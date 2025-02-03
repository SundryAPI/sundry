use async_openai::error::OpenAIError;
use models::source::{Source, Sources};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::{future::Future, pin::Pin};
use thiserror::Error;

use crate::{
    model::{CompletionError, MODELS},
    v1::AuthData,
};

pub mod step;

use step::{query::MaybeQueriedSourceData, Step};

const MAX_EXECUTE_DEPTH: u32 = 5;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum Confidence {
    Doubtful,
    Tentative,
    Optimistic,
    Certain,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlanSuccessResponse {
    pub confidence: Confidence,
    pub data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_message: Option<String>,
    #[cfg(feature = "integration")]
    pub plan: Plan,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlanFailureResponse {
    pub confidence: Confidence,
    pub user_message: String,
    #[cfg(feature = "integration")]
    pub plan: Plan,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PlanResponse {
    Success(PlanSuccessResponse),
    Failure(PlanFailureResponse),
}

#[cfg(feature = "integration")]
impl PlanResponse {
    pub fn to_success(self) -> Option<PlanSuccessResponse> {
        match self {
            PlanResponse::Success(plan_success_response) => Some(plan_success_response),
            PlanResponse::Failure(_) => None,
        }
    }

    pub fn to_failure(self) -> Option<PlanFailureResponse> {
        match self {
            PlanResponse::Success(_) => None,
            PlanResponse::Failure(plan_failure_response) => Some(plan_failure_response),
        }
    }
}

#[derive(Error, Debug)]
pub enum PlanError {
    #[error(transparent)]
    OpenAIAPI(#[from] OpenAIError),
    #[error(transparent)]
    OpenAISerde(#[from] serde_json::Error),
    #[error("no content in OpenAI message")]
    OpenAINoMessageContent,
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error("confidence in {0}: {1}")]
    Confidence(String, String),
    #[error("Error: {0}")]
    EmptyDataSelectionForReturn(String),
    #[error("too many planner steps")]
    TooManyPlannerSteps,
    #[error("trying to summarize nonexistent data")]
    SummarizedNonexistentData,
    #[error("empty data selection in return with empty user message")]
    EmptyDataSelectionForReturnWithEmptyUserMessage,
    #[error("doubful confidence but performing a query")]
    DoubtfulConfidenceForQueryStep,
    #[error("requesting access to non-synced resource: {0}")]
    RequestingAccessToNonSyncedSource(Source),
    #[error("too many steps querying: {0}")]
    TooManyStepsQuerying(Source),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error("Failed to parse response: {0}")]
    LLMResponseParse(String),
}

impl From<CompletionError> for PlanError {
    fn from(error: CompletionError) -> Self {
        match error {
            CompletionError::OpenAI(e) => PlanError::OpenAIAPI(e),
            CompletionError::Serde(e) => PlanError::OpenAISerde(e),
            CompletionError::NoMessageContent => PlanError::OpenAINoMessageContent,
            CompletionError::Reqwest(e) => PlanError::Reqwest(e),
            CompletionError::LLMResponseParse(e) => PlanError::LLMResponseParse(e),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Plan {
    pub query: String,
    pub executed_steps: Vec<Step>,
    pub past_steps_data: Vec<QueryStepOutput>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QueryStepOutput {
    pub query: String,
    pub data: MaybeQueriedSourceData,
    pub explanation: String,
}

impl Plan {
    pub fn new(query: &str) -> Self {
        Self {
            query: query.to_string(),
            executed_steps: vec![],
            past_steps_data: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct QueryPlannerInput<'a, 'b, 'c> {
    pub user_query: &'a str,
    pub past_steps: &'b [QueryStepOutput],
    pub available_sources: &'c Sources,
}

#[derive(Debug, Clone)]
pub struct PlanManager {
    pub plan: Plan,
    pub execute_depth: u32,
}

impl PlanManager {
    pub fn new(query: &str) -> Self {
        Self {
            plan: Plan::new(query),
            execute_depth: 0,
        }
    }

    pub fn execute<'a, 'b: 'a>(
        &'a mut self,
        auth_data: &'b AuthData,
        pool: &'b PgPool,
    ) -> Pin<Box<dyn Future<Output = Result<PlanResponse, PlanError>> + 'a + Send>> {
        Box::pin(async move {
            if self.execute_depth > MAX_EXECUTE_DEPTH {
                return Err(PlanError::TooManyPlannerSteps);
            }
            self.execute_depth += 1;

            let mut step: Step = MODELS
                .query_planner
                .chat_with_value(
                    include_str!("../../prompts/v1.1/planner_v1.txt"),
                    &QueryPlannerInput {
                        user_query: &self.plan.query,
                        past_steps: &self.plan.past_steps_data,
                        available_sources: &auth_data.available_sources,
                    },
                )
                .await?;

            // Rust borrow rules make us do this
            #[cfg(feature = "integration")]
            let step_clone = step.clone();

            match &mut step {
                Step::Query(query) => {
                    // Runs any summarize tool calls used in the query
                    let processed_query =
                        query.process_summarize(&self.plan.past_steps_data).await?;

                    let query_response = processed_query.execute(auth_data, pool).await?;

                    self.plan.past_steps_data.push(QueryStepOutput {
                        query: query.query.clone(),
                        data: query_response,
                        explanation: query.explanation.clone(),
                    });

                    self.plan.executed_steps.push(step);

                    self.execute(auth_data, pool).await
                }
                Step::Return(ret) => {
                    #[cfg(feature = "integration")]
                    self.plan.executed_steps.push(step_clone);

                    if let Some(data_selection) = ret.data_selection.take() {
                        let output = ret
                            .execute(&self.plan.past_steps_data, &data_selection)
                            .await?;
                        Ok(PlanResponse::Success(PlanSuccessResponse {
                            confidence: ret.confidence,
                            data: output,
                            user_message: ret.user_message.take(),
                            #[cfg(feature = "integration")]
                            plan: self.plan.clone(),
                        }))
                    } else {
                        Ok(PlanResponse::Failure(PlanFailureResponse {
                            confidence: ret.confidence,
                            user_message: ret.user_message.take().ok_or(
                                PlanError::EmptyDataSelectionForReturnWithEmptyUserMessage,
                            )?,
                            #[cfg(feature = "integration")]
                            plan: self.plan.clone(),
                        }))
                    }
                }
            }
        })
    }
}
