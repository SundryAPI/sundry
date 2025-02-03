use std::collections::HashMap;

use models::source::{Source, SourceItem, Sources};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use slack::SlackExecutor;
use sqlx::PgPool;

pub mod github;
pub mod google_docs;
pub mod slack;

use crate::{
    plan::{Confidence, PlanError, QueryStepOutput},
    v1::AuthData,
};

use super::utils::summarize_data;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QueryFailure {
    error: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QueriedData {
    pub assumptions_made_during_query: Option<String>,
    pub queried_data: Vec<Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum QueriedSourceDataOrError {
    QueriedData(QueriedData),
    QueryFailure(QueryFailure),
}

pub type MaybeQueriedSourceData = HashMap<Source, HashMap<SourceItem, QueriedSourceDataOrError>>;
pub type QueriedSourceData = HashMap<Source, HashMap<SourceItem, QueriedData>>;

#[derive(Debug, Clone)]
pub struct QueryExecuteResponse {
    pub data: QueriedSourceData,
    pub assumptions: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Query {
    pub query: String,
    pub confidence: Confidence,
    pub explanation: String,
    pub targets: Sources,
}

impl Query {
    pub async fn process_summarize(
        &self,
        past_steps_data: &[QueryStepOutput],
    ) -> Result<Self, PlanError> {
        let re = Regex::new(r#"\$\{summarize\(\$(\d+)(?:,\s*"([^"]+)"\s*,\s*"([^"]+)"\s*)?\)\}"#)
            .unwrap();
        let mut result = String::from(&self.query);
        let mut replacements = Vec::new();

        for cap in re.captures_iter(&self.query) {
            let step_str = &cap[1];
            let step_num: usize = step_str
                .parse()
                .map_err(|_| PlanError::SummarizedNonexistentData)?;
            let source = cap.get(2).map(|m| m.as_str());
            let data_type = cap.get(3).map(|m| m.as_str());

            match (source, data_type) {
                (Some(s), Some(d)) => {
                    let source: Source = s
                        .parse()
                        .map_err(|_| PlanError::SummarizedNonexistentData)?;
                    let source_item: SourceItem = format!("{s}: {d}")
                        .parse()
                        .map_err(|_| PlanError::SummarizedNonexistentData)?;
                    let data = past_steps_data
                        .get(step_num)
                        .ok_or(PlanError::SummarizedNonexistentData)?
                        .data
                        .get(&source)
                        .ok_or(PlanError::SummarizedNonexistentData)?
                        .get(&source_item)
                        .ok_or(PlanError::SummarizedNonexistentData)?;
                    match data {
                        QueriedSourceDataOrError::QueriedData(queried_data) => {
                            let summarized_data = summarize_data(
                                &serde_json::to_string(&queried_data.queried_data).unwrap(),
                            )
                            .await?;
                            replacements.push((cap.get(0).unwrap().range(), summarized_data));
                        }
                        QueriedSourceDataOrError::QueryFailure(_) => {
                            return Err(PlanError::SummarizedNonexistentData)
                        }
                    }
                }
                _ => {
                    // Filter out the query failures and assumptions from the data to be summarized
                    let data = &past_steps_data
                        .get(step_num)
                        .ok_or(PlanError::SummarizedNonexistentData)?
                        .data;
                    let data: HashMap<Source, HashMap<SourceItem, Vec<Value>>> = data
                        .iter()
                        .map(|(source, data)| {
                            let map: HashMap<SourceItem, Vec<Value>> = data
                                .iter()
                                .filter_map(|(source_item, maybe_queried_data)| {
                                    match maybe_queried_data {
                                        QueriedSourceDataOrError::QueriedData(queried_data) => {
                                            Some((*source_item, queried_data.queried_data.clone()))
                                        }
                                        QueriedSourceDataOrError::QueryFailure(_) => None,
                                    }
                                })
                                .collect();
                            (*source, map)
                        })
                        .collect();

                    let summarized_data =
                        summarize_data(&serde_json::to_string(&data).unwrap()).await?;
                    replacements.push((cap.get(0).unwrap().range(), summarized_data));
                }
            }
        }

        // Because we're replacing substrings, we should do it from the end of the
        // string forward to avoid messing up the ranges.
        for (range, summary) in replacements.into_iter().rev() {
            result.replace_range(range, &summary);
        }

        Ok(Self {
            query: result,
            confidence: self.confidence,
            explanation: self.explanation.clone(),
            targets: self.targets.clone(),
        })
    }

    pub async fn execute(
        &self,
        auth_data: &AuthData,
        pool: &PgPool,
    ) -> Result<MaybeQueriedSourceData, PlanError> {
        if self.confidence == Confidence::Doubtful {
            return Err(PlanError::DoubtfulConfidenceForQueryStep);
        }

        let mut source_data = HashMap::new();

        // TODO: Run all at once instead of one after the other
        for (source, source_items) in &self.targets {
            source_data.insert(*source, HashMap::new());

            for source_item in source_items.iter() {
                let response = match source_item {
                    SourceItem::Slack(_) => {
                        SlackExecutor::new(&self.query, auth_data, pool)
                            .await?
                            .execute()
                            .await?
                    }
                    SourceItem::GitHub(_) => github::execute(&self.query, auth_data, pool).await?,
                    SourceItem::Google(item) => match item {
                        models::source::GoogleItem::Docs => {
                            google_docs::execute(&self.query, auth_data, pool).await?
                        }
                    },
                    SourceItem::Jira(_jira_item) => todo!(),
                };

                // SAFTEY: We always insert `source` above
                source_data
                    .get_mut(source)
                    .unwrap()
                    .insert(*source_item, response);
            }
        }

        Ok(source_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queried_source_data_serialization() {
        let mut source_data: MaybeQueriedSourceData = HashMap::new();

        let queried_data = QueriedData {
            assumptions_made_during_query: Some("Assumed recent issues".to_string()),
            queried_data: vec![
                serde_json::json!({"id": 1, "title": "Bug report"}),
                serde_json::json!({"id": 2, "title": "Feature request"}),
            ],
        };

        let error_data = QueryFailure {
            error: "Query failed".to_string(),
        };

        let mut github_map = HashMap::new();
        github_map.insert(
            SourceItem::GitHub(models::source::GitHubItem::Issues),
            QueriedSourceDataOrError::QueriedData(queried_data.clone()),
        );
        github_map.insert(
            SourceItem::GitHub(models::source::GitHubItem::PullRequests),
            QueriedSourceDataOrError::QueryFailure(error_data),
        );

        source_data.insert(Source::GitHub, github_map);

        let value = serde_json::to_value(&source_data).expect("Failed to serialize");
        assert_eq!(
            value,
            serde_json::json!({"GitHub":{"PullRequests":{"error":"Query failed"},"Issues":{"assumptions_made_during_query":"Assumed recent issues","queried_data":[{"id":1,"title":"Bug report"},{"id":2,"title":"Feature request"}]}}})
        );
    }
}
