use models::{source::Source, user::github_username_for_user_id};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{types::Json, PgPool};

use crate::{
    model::MODELS,
    plan::{
        step::{
            query::{QueriedData, QueriedSourceDataOrError},
            utils::transform_sql_query,
        },
        Confidence, PlanError,
    },
    v1::AuthData,
};

use super::QueryFailure;

const GATED_TABLE_MAPPINGS: [(&str, &str); 6] = [
    ("github.repositories", "github.org_repositories"),
    ("github.issues", "github.org_issues"),
    ("github.comments", "github.org_comments"),
    ("github.pull_requests", "github.org_pull_requests"),
    ("github.issue_labels", "github.org_issue_labels"),
    ("github.issue_assignees", "github.org_issue_assignees"),
];

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GitHubText2SQLSuccessResponse {
    pub sql: String,
    pub confidence: Confidence,
    pub assumptions: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GitHubText2SQLFailureResponse {
    pub error: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum GitHubText2SQLResponse {
    Success(GitHubText2SQLSuccessResponse),
    Failure(GitHubText2SQLFailureResponse),
}

#[derive(Debug, Clone, Serialize)]
pub struct GitHubQueryInput<'a, 'b> {
    query: &'a str,
    user: &'b str,
}

pub async fn execute(
    query: &str,
    auth_data: &AuthData,
    pool: &PgPool,
) -> Result<QueriedSourceDataOrError, PlanError> {
    let prompt = GitHubQueryInput {
        query,
        user: &github_username_for_user_id(auth_data.user_key.user_id, pool)
            .await?
            .ok_or(PlanError::RequestingAccessToNonSyncedSource(Source::GitHub))?,
    };

    let text_2_sql: GitHubText2SQLResponse = MODELS
        .text_2_sql
        .chat_with_value(
            include_str!("../../../../prompts/v1/github_v4.txt"),
            &prompt,
        )
        .await?;

    match text_2_sql {
        GitHubText2SQLResponse::Success(text2_sqlsuccess_response) => {
            let sql_query =
                transform_sql_query(&text2_sqlsuccess_response.sql, &GATED_TABLE_MAPPINGS);

            tracing::debug!("THE GATED SQL QUERY:\n{sql_query}\n");
            let rows: Vec<(Json<Value>,)> = sqlx::query_as("SELECT execute_org_query($1, $2)")
                .bind(auth_data.user_key.organization_id)
                .bind(sql_query)
                .fetch_all(pool)
                .await?;
            let result: Vec<Value> = rows.into_iter().map(|(json,)| json.0).collect();

            Ok(QueriedSourceDataOrError::QueriedData(QueriedData {
                queried_data: result,
                assumptions_made_during_query: text2_sqlsuccess_response.assumptions,
            }))
        }
        GitHubText2SQLResponse::Failure(git_hub_text2_sqlfailure_response) => {
            Ok(QueriedSourceDataOrError::QueryFailure(QueryFailure {
                error: git_hub_text2_sqlfailure_response.error,
            }))
        }
    }
}
