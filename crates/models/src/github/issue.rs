use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use sqlx::{Postgres, Transaction};

use super::{label::Label, repository::Repository, user::User};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub id: i64,
    pub repository_url: String,
    pub url: String,
    pub comments_url: String,
    pub number: i32,
    pub state: String,
    pub title: String,
    pub body: Option<String>,
    pub user: User,
    pub labels: Vec<Label>,
    pub assignees: Vec<User>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub closed_by: Option<User>,
}

#[cfg(feature = "ssr")]
impl Issue {
    pub async fn get_id_by_url(
        issue_url: &str,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Option<i64>, sqlx::Error> {
        sqlx::query!("SELECT id FROM github.issues WHERE url = $1", issue_url)
            .fetch_optional(&mut **transaction)
            .await
            .map(|row| row.map(|r| r.id))
    }

    pub async fn insert<'a>(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<bool, sqlx::Error> {
        // lookup repository by url
        let repository_id =
            match Repository::get_id_by_url(&self.repository_url, &mut **transaction).await? {
                Some(repository_id) => repository_id,
                None => return Ok(false),
            };

        // insert all foreign key references first
        for label in &self.labels {
            label.insert(&mut **transaction).await?;
        }
        self.user.insert(&mut **transaction).await?;
        for assignee in &self.assignees {
            assignee.insert(&mut **transaction).await?;
        }
        if let Some(closed_by) = &self.closed_by {
            closed_by.insert(&mut **transaction).await?;
        }

        sqlx::query!(
            "INSERT INTO github.issues (
                id,
                repository_id,
                url,
                comments_url,
                number,
                state,
                title,
                body,
                user_id,
                created_at,
                updated_at,
                closed_at,
                closed_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            ON CONFLICT DO NOTHING",
            self.id,
            repository_id,
            self.url,
            self.comments_url,
            self.number,
            self.state,
            self.title,
            self.body,
            self.user.id,
            self.created_at.naive_utc(),
            self.updated_at.naive_utc(),
            self.closed_at.map(|t| t.naive_utc()),
            self.closed_by.as_ref().map(|u| u.id),
        )
        .execute(&mut **transaction)
        .await?;

        // insert the join tables
        for label in &self.labels {
            sqlx::query!(
                "INSERT INTO github.issue_labels (issue_id, label_id)
                VALUES ($1, $2)
                ON CONFLICT DO NOTHING",
                self.id,
                label.id,
            )
            .execute(&mut **transaction)
            .await?;
        }

        for assignee in &self.assignees {
            sqlx::query!(
                "INSERT INTO github.issue_assignees (issue_id, user_id)
                VALUES ($1, $2)
                ON CONFLICT DO NOTHING",
                self.id,
                assignee.id,
            )
            .execute(&mut **transaction)
            .await?;
        }

        Ok(true)
    }
}
