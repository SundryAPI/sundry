use serde::{Deserialize, Serialize};

use crate::{model::MODELS, plan::PlanError};

use super::query::QueriedSourceData;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SummarizeResponse {
    summarized_data: String,
}

pub async fn summarize_data(data: &str) -> Result<String, PlanError> {
    let summarize_response: SummarizeResponse = MODELS
        .summarize
        .chat_with_value(
            include_str!("../../../prompts/v1/summarize_v1.txt"),
            &serde_json::json!({
                "data": data
            }),
        )
        .await?;
    Ok(summarize_response.summarized_data)
}

pub fn transform_sql_query(query: &str, replacements: &[(&str, &str)]) -> String {
    let mut result = query.to_string();

    for (pattern, replacement) in replacements {
        result = result.replace(pattern, replacement);
    }

    result
}

pub fn format_queried_source_data(data: &QueriedSourceData) -> String {
    data.iter()
        .map(|(_source, items)| {
            items
                .iter()
                .map(|(_item, data)| serde_json::to_string(&data.queried_data).unwrap())
                .collect::<Vec<_>>()
                .join("\n\n")
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}
