use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use sqlx::{Postgres, Transaction};

use super::issue::Issue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequest {
    pub issue_url: String,
    pub merged_at: Option<DateTime<Utc>>,
}

#[cfg(feature = "ssr")]
impl PullRequest {
    pub async fn insert(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<bool, sqlx::Error> {
        // lookup issue by url
        let issue_id = match Issue::get_id_by_url(&self.issue_url, transaction).await? {
            Some(issue_id) => issue_id,
            None => return Ok(false),
        };

        sqlx::query!(
            "INSERT INTO github.pull_requests (id, merged_at)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING",
            issue_id,
            self.merged_at.map(|t| t.naive_utc()),
        )
        .execute(&mut **transaction)
        .await?;

        Ok(true)
    }
}
