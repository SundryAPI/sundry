use serde::{Deserialize, Serialize};

use crate::{
    model::{CompletionError, MODELS},
    plan::Confidence,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformedData {
    pub transformed_data: String,
    pub confidence: Confidence,
}

pub async fn transform_data<T: Serialize>(
    instruction: &str,
    data: &T,
) -> Result<TransformedData, CompletionError> {
    MODELS
        .transform
        .chat_with_value(
            include_str!("../prompts/v1/transform_v1.txt"),
            &serde_json::json!({
                "instruction": instruction,
                "data": data
            }),
        )
        .await
}
