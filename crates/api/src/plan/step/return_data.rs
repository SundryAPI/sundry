use std::collections::HashMap;

use models::source::{Source, SourceItem, Sources};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    plan::{Confidence, PlanError, QueryStepOutput},
    utils::transform_data,
};

#[derive(Debug, Clone)]
pub struct ReturnDataResponse {
    pub confidence: Confidence,
    pub data: String,
    pub user_message: Option<String>,
}

pub type ReturnDataSelection = HashMap<String, Sources>;
pub type ReturnExecuteData = HashMap<Source, HashMap<SourceItem, Vec<Value>>>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReturnData {
    pub data_selection: Option<ReturnDataSelection>,
    pub transform_instruction: Option<String>,
    pub confidence: Confidence,
    pub user_message: Option<String>,
}

impl ReturnData {
    pub async fn execute(
        &self,
        past_steps_data: &[QueryStepOutput],
        data_selection: &ReturnDataSelection,
    ) -> Result<String, PlanError> {
        let mut data: ReturnExecuteData = ReturnExecuteData::new();

        for (step_index, selected_sources) in data_selection {
            if let Ok(step_index) = step_index.parse::<usize>() {
                if let Some(step_data) = past_steps_data.get(step_index) {
                    for (selected_source, selected_source_items) in selected_sources {
                        if let Some(step_source_items) = step_data.data.get(selected_source) {
                            for selected_source_item in selected_source_items {
                                if let Some(step_item_data) =
                                    step_source_items.get(selected_source_item)
                                {
                                    let source_data = match data.get_mut(selected_source) {
                                        Some(data) => data,
                                        None => {
                                            data.insert(*selected_source, HashMap::new());
                                            data.get_mut(selected_source).unwrap()
                                        }
                                    };

                                    let item_data = match source_data.get_mut(selected_source_item)
                                    {
                                        Some(vec) => vec,
                                        None => {
                                            source_data.insert(*selected_source_item, vec![]);
                                            source_data.get_mut(selected_source_item).unwrap()
                                        }
                                    };

                                    if let super::query::QueriedSourceDataOrError::QueriedData(
                                        data,
                                    ) = step_item_data
                                    {
                                        item_data.append(&mut data.queried_data.clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(match &self.transform_instruction {
            Some(instruction) => {
                let transformed_data = transform_data(instruction, &data).await?;
                transformed_data.transformed_data
            }
            None => serde_json::to_string(&data).unwrap(),
        })
    }
}
