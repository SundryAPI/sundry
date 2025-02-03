use async_openai::{
    config::OpenAIConfig,
    error::OpenAIError,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
        ChatCompletionRequestUserMessage, CreateChatCompletionResponse,
    },
    Client,
};
use regex::Regex;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::sync::LazyLock;
use thiserror::Error;
use tracing::debug;

pub static MODELS: LazyLock<ModelConfig> = LazyLock::new(|| {
    let config_str = std::fs::read_to_string(
        std::env::var("API_MODEL_CONFIG_PATH").expect("no API_MODEL_CONFIG_PATH env variable set"),
    )
    .expect("Failed to read model_config.toml");
    toml::from_str(&config_str).expect("Failed to parse config.toml")
});

#[derive(Clone, Deserialize)]
pub struct ModelConfig {
    pub query_planner: Model,
    pub text_2_sql: Model,
    pub transform: Model,
    pub summarize: Model,
}

#[derive(Clone, Deserialize)]
pub struct ModelDefinition {
    pub key_name: String,
    pub base_url: String,
    pub model: String,
}

#[derive(Clone, Deserialize)]
pub struct GeminiModelDefinition {
    model: String,
    key_name: String,
}

#[derive(Clone, Deserialize)]
#[serde(tag = "type")]
pub enum Model {
    #[serde(rename = "anthropic")]
    Anthropic(ModelDefinition),
    #[serde(rename = "openai")]
    OpenAI(ModelDefinition),
    #[serde(rename = "gemini")]
    Gemini(GeminiModelDefinition),
}

#[derive(Error, Debug)]
pub enum CompletionError {
    #[error(transparent)]
    OpenAI(#[from] OpenAIError),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error("no content in message")]
    NoMessageContent,
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error("Failed to parse response: {0}")]
    LLMResponseParse(String),
}

pub fn maybe_extract_code_from_markdown(text: &str) -> String {
    let re = Regex::new(r"(?s)\s*```(\w+)?\s*(.*?)\s*```\s*$").unwrap();

    if let Some(captures) = re.captures(text) {
        captures
            .get(2)
            .map_or("", |m| m.as_str())
            .trim()
            .to_string()
    } else {
        text.to_string()
    }
}

impl Model {
    pub async fn chat_with_value<T: Serialize, F: DeserializeOwned + Serialize>(
        &self,
        system_prompt: &str,
        user_message: &T,
    ) -> Result<F, CompletionError> {
        let message = self.chat(system_prompt, user_message).await?;

        let extracted_message = maybe_extract_code_from_markdown(&message);
        // First extract it as a value to handle duplicate keys instead of erroring
        let value: serde_json::Value = serde_json::from_str(&extracted_message)?;
        let resp: F = serde_json::from_value(value)?;
        debug!(
            "LLM FORMATTED RESPONSE:\n\n{}\n\n",
            serde_json::to_string_pretty(&resp).unwrap()
        );
        Ok(resp)
    }

    pub async fn chat<T: Serialize>(
        &self,
        system_prompt: &str,
        user_message: &T,
    ) -> Result<String, CompletionError> {
        match self {
            Model::Anthropic(_model_config) => unimplemented!("anthropic not implemented yet"),
            Model::OpenAI(model_config) => {
                let config = OpenAIConfig::new()
                    .with_api_key(std::env::var(&model_config.key_name).unwrap_or_else(|_| {
                        panic!("missing env model key: {}", model_config.key_name)
                    }))
                    .with_api_base(&model_config.base_url);
                let client = Client::with_config(config);
                let chat_client = client.chat();

                debug!(
                    "LLM USER MESSAGE:\n\n{}\n\n",
                    serde_json::to_string_pretty(user_message).unwrap()
                );
                let response = chat_client
                    .create(async_openai::types::CreateChatCompletionRequest {
                        model: model_config.model.to_string(),
                        messages: vec![
                            ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage {
                                content: async_openai::types::ChatCompletionRequestSystemMessageContent::Text(
                                    system_prompt.to_string(),
                                ),
                                ..Default::default()
                            }),
                            ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
                                content: async_openai::types::ChatCompletionRequestUserMessageContent::Text(
                                    serde_json::to_string(user_message).unwrap(),
                                ),
                                ..Default::default()
                            }),
                        ],
                        temperature: Some(0.3),
                        ..Default::default()
                    })
                    .await;
                debug!("LLM RESPONSE:\n\n{:?}\n\n", response);

                let response = response?;
                let response = response.choices[0]
                    .message
                    .content
                    .as_ref()
                    .ok_or(CompletionError::NoMessageContent)?;
                Ok(response.to_string())
            }
            Model::Gemini(gemini_model_definition) => {
                call_gemini(gemini_model_definition, system_prompt, user_message).await
            }
        }
    }
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: Content,
}

#[derive(Deserialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Deserialize)]
struct Part {
    text: String,
}

async fn call_gemini<T: Serialize>(
    model_config: &GeminiModelDefinition,
    system_prompt: &str,
    message: &T,
) -> Result<String, CompletionError> {
    let client = reqwest::Client::new();
    let response = client
        .post(format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            model_config.model,
            std::env::var(&model_config.key_name)
                .unwrap_or_else(|_| { panic!("missing env model key: {}", model_config.key_name) })
        ))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "system_instruction": {
                "parts": {
                    "text": system_prompt
                }
            },
            "contents": [
                {
                    "role": "user",
                    "parts": [{
                        "text": serde_json::to_string(&message).unwrap()
                    }]
                }
            ]
        }))
        .send()
        .await?
        .text()
        .await?;

    match serde_json::from_str::<GeminiResponse>(&response) {
        Ok(parsed) => {
            let text = &parsed.candidates[0].content.parts[0].text;
            tracing::debug!("LLM RESPONSE:\n\n{}\n\n", text);
            Ok(text.clone())
        }
        Err(_) => Err(CompletionError::LLMResponseParse(response)),
    }
}
