use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use serde_json::Value;
#[cfg(feature = "ssr")]
use sqlx::{Decode, PgPool};

#[cfg(feature = "ssr")]
#[derive(thiserror::Error, Debug)]
pub enum RequestError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

#[cfg_attr(feature = "ssr", derive(Decode))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub id: i64,
    pub user_api_key_id: i64,
    pub application_api_key_id: i64,
    pub version: i32,
    pub request_body: Option<Value>,
    pub response_body: Option<Value>,
    pub response_status: Option<i32>,
    pub duration_ms: Option<i32>,
    pub created_at: NaiveDateTime,
}

#[cfg(feature = "ssr")]
impl Request {
    pub async fn create(
        user_api_key_id: i64,
        application_api_key_id: i64,
        version: i32,
        request_body: &Value,
        pool: &PgPool,
    ) -> Result<Self, RequestError> {
        Ok(sqlx::query_as!(Request, "INSERT INTO api_requests (user_api_key_id, application_api_key_id, version, request_body) VALUES ($1, $2, $3, $4) RETURNING *", user_api_key_id, application_api_key_id, version, request_body).fetch_one(pool).await?)
    }

    pub async fn update_in_database(&self, pool: &PgPool) -> Result<(), RequestError> {
        sqlx::query!("UPDATE api_requests SET user_api_key_id = $1, application_api_key_id = $2, version = $3, request_body = $4, response_body = $5, response_status = $6, duration_ms = $7", self.user_api_key_id, self.application_api_key_id, self.version, self.request_body, self.response_body, self.response_status, self.duration_ms).execute(pool).await?;
        Ok(())
    }

    pub async fn get_with_id(id: i64, pool: &PgPool) -> Result<Option<Self>, RequestError> {
        Ok(
            sqlx::query_as!(Request, "SELECT * FROM api_requests WHERE id = $1", id)
                .fetch_optional(pool)
                .await?,
        )
    }
}
