use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[cfg(feature = "ssr")]
use sqlx::{Decode, PgPool};

#[cfg_attr(feature = "ssr", derive(Decode))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: i64,
    pub user_id: Option<i64>,
    pub logged_in: bool,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub data: serde_json::Value,
}

#[cfg(feature = "ssr")]
impl Session {
    pub async fn create(
        user_id: Option<i64>,
        data: Option<serde_json::Value>,
        pool: &PgPool,
    ) -> Result<Session, sqlx::Error> {
        sqlx::query_as!(Session, "INSERT INTO sessions (user_id, logged_in, expires_at, data) VALUES ($1, true, CURRENT_TIMESTAMP + '5 DAY', $2) RETURNING *", user_id, data.unwrap_or_else(|| json!({}))).fetch_one(pool).await
    }

    pub async fn logout(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE sessions SET logged_in = false WHERE id = $1",
            self.id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn get_with_id(id: i64, pool: &PgPool) -> Result<Option<Session>, sqlx::Error> {
        sqlx::query_as!(Session, "SELECT * FROM sessions WHERE id = $1", id)
            .fetch_optional(pool)
            .await
    }

    pub async fn is_expired(&self, pool: &PgPool) -> Result<bool, sqlx::Error> {
        let session = sqlx::query_as!(Session, "SELECT * FROM sessions WHERE id = $1", self.id)
            .fetch_one(pool)
            .await?;
        Ok(session.expires_at < chrono::Local::now().naive_local())
    }

    pub async fn update_data(&self, pool: &PgPool, data: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE sessions SET data = $1 WHERE id = $2",
            serde_json::to_value(data).unwrap(),
            self.id
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
