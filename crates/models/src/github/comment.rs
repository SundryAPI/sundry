use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use sqlx::{Postgres, Transaction};

use super::{issue::Issue, user::User};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: i64,
    pub issue_url: String,
    pub user: User,
    pub url: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg(feature = "ssr")]
impl Comment {
    pub async fn insert(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<bool, sqlx::Error> {
        // insert foreign keys first
        self.user.insert(&mut **transaction).await?;

        // lookup issue by url
        let issue_id = match Issue::get_id_by_url(&self.issue_url, transaction).await? {
            Some(issue_id) => issue_id,
            None => return Ok(false),
        };

        sqlx::query!(
            "INSERT INTO github.comments (id, issue_id, user_id, url, body, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT DO NOTHING",
            self.id,
            issue_id,
            self.user.id,
            self.url,
            self.body,
            self.created_at.naive_utc(),
            self.updated_at.naive_utc(),
        )
        .execute(&mut **transaction)
        .await?;

        Ok(true)
    }
}
