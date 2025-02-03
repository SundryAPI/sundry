use std::{future::Future, pin::Pin};

use models::{source::Source, user::slack_user_id_for_user_id};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{types::Json, PgPool};

use crate::{
    model::MODELS,
    plan::{step::utils::transform_sql_query, Confidence, PlanError},
    v1::AuthData,
};

use super::{QueriedData, QueriedSourceDataOrError, QueryFailure};

const MAX_ALLOWED_STEPS: u8 = 5;

const GATED_TABLE_MAPPINGS: [(&str, &str); 4] = [
    ("slack.channels", "slack.org_user_channels"),
    ("slack.channel_members", "slack.org_user_channel_members"),
    ("slack.users", "slack.org_user_users"),
    ("slack.messages", "slack.org_user_messages"),
];

#[derive(Debug, Clone, Serialize)]
pub struct SlackQueryInput<'a, 'b, 'c> {
    pub query: &'a str,
    pub user_id: &'b str,
    pub past_steps: &'c [SlackQueryWithData],
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "action")]
pub enum SlackStep {
    Query(SlackQuery),
    Return(SlackReturn),
}

#[cfg(feature = "integration")]
impl SlackStep {
    pub fn as_query(&self) -> Option<&SlackQuery> {
        match self {
            SlackStep::Query(query) => Some(query),
            SlackStep::Return(_) => None,
        }
    }

    pub fn as_return(&self) -> Option<&SlackReturn> {
        match self {
            SlackStep::Query(_) => None,
            SlackStep::Return(ret) => Some(ret),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SlackQuery {
    pub sql_query: String,
    pub confidence: Confidence,
    pub explanation: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SlackQueryWithData {
    pub sql_query: String,
    pub confidence: Confidence,
    pub explanation: String,
    pub data: Vec<Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SlackReturn {
    pub data_selection: Option<Vec<usize>>,
    pub transform_instruction: Option<String>,
    pub user_message: Option<String>,
    pub confidence: Confidence,
}

pub struct SlackExecutor {
    query: String,
    auth_data: AuthData,
    pool: PgPool,
    past_steps: Vec<SlackQueryWithData>,
    user_id: String,
    steps: u8,
}

impl SlackExecutor {
    pub async fn new(query: &str, auth_data: &AuthData, pool: &PgPool) -> Result<Self, PlanError> {
        Ok(Self {
            query: query.to_string(),
            auth_data: auth_data.clone(),
            pool: pool.clone(),
            past_steps: vec![],
            user_id: slack_user_id_for_user_id(auth_data.user_key.user_id, pool)
                .await?
                .ok_or(PlanError::RequestingAccessToNonSyncedSource(Source::Slack))?,
            steps: 0,
        })
    }

    pub fn execute<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<QueriedSourceDataOrError, PlanError>> + 'a + Send>>
    {
        Box::pin(async move {
            if self.steps > MAX_ALLOWED_STEPS {
                return Err(PlanError::TooManyStepsQuerying(Source::Slack));
            }
            self.steps += 1;

            let prompt = SlackQueryInput {
                query: &self.query,
                user_id: &self.user_id,
                past_steps: &self.past_steps,
            };

            let step: SlackStep = MODELS
                .text_2_sql
                .chat_with_value(
                    include_str!("../../../../prompts/v1.1/slack_v1.txt"),
                    &prompt,
                )
                .await?;

            match step {
                SlackStep::Query(slack_query) => {
                    let sql_query =
                        transform_sql_query(&slack_query.sql_query, &GATED_TABLE_MAPPINGS);

                    // Replace occurrences of `user` with `"user"`
                    let re = Regex::new(r"(?P<before>\W)user(?P<after>\W)").unwrap();

                    // Replace occurrences of `user` with `"user"`
                    let sql_query = re.replace_all(&sql_query, |caps: &regex::Captures| {
                        format!("{}\"user\"{}", &caps["before"], &caps["after"])
                    });

                    tracing::debug!(
                        "FULL QUERY:\nSELECT execute_org_user_query({}, '{}', '{}')",
                        self.auth_data.user_key.organization_id,
                        self.user_id,
                        sql_query.replace("'", "''")
                    );

                    tracing::debug!("THE GATED SQL QUERY:\n{sql_query}\n");
                    let rows: Vec<(Json<Value>,)> =
                        sqlx::query_as("SELECT execute_org_user_query($1, $2, $3)")
                            .bind(self.auth_data.user_key.organization_id)
                            .bind(&self.user_id)
                            .bind(sql_query)
                            .fetch_all(&self.pool)
                            .await?;

                    let result: Vec<Value> = rows.into_iter().map(|(json,)| json.0).collect();
                    self.past_steps.push(SlackQueryWithData {
                        sql_query: slack_query.sql_query,
                        confidence: slack_query.confidence,
                        explanation: slack_query.explanation,
                        data: result,
                    });
                    self.execute().await
                }
                SlackStep::Return(slack_return) => Ok(
                    match (slack_return.data_selection, slack_return.user_message) {
                        (None, None) => QueriedSourceDataOrError::QueryFailure(QueryFailure {
                            error: "unkown error while performing query".to_string(),
                        }),
                        (None, Some(user_message)) => {
                            QueriedSourceDataOrError::QueryFailure(QueryFailure {
                                error: user_message,
                            })
                        }
                        ref data @ (Some(ref data_selection), _) => {
                            let queried_data: Vec<Value> = data_selection
                                .iter()
                                .filter_map(|index| {
                                    self.past_steps.get(*index).map(|step| step.data.clone())
                                })
                                .flatten()
                                .collect();
                            QueriedSourceDataOrError::QueriedData(QueriedData {
                                assumptions_made_during_query: data.1.clone(),
                                queried_data,
                            })
                        }
                    },
                ),
            }
        })
    }
}
