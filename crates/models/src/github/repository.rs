use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use sqlx::{Executor, Postgres, Transaction};

use super::user::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub id: i64,
    pub full_name: String,
    pub owner: User,
    pub private: bool,
    pub url: String,
}

#[cfg(feature = "ssr")]
impl Repository {
    pub async fn get_id_by_url<'a, E>(
        repository_url: &str,
        executor: E,
    ) -> Result<Option<i64>, sqlx::Error>
    where
        E: Executor<'a, Database = Postgres>,
    {
        sqlx::query!(
            "SELECT id FROM github.repositories WHERE url = $1",
            repository_url
        )
        .fetch_optional(executor)
        .await
        .map(|row| row.map(|r| r.id))
    }

    pub async fn insert(
        &self,
        app_installation_id: i64,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<(), sqlx::Error> {
        self.owner.insert(&mut **transaction).await?;

        sqlx::query!(
            "INSERT INTO github.repositories (id, full_name, owner_id, private, url)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT DO NOTHING",
            self.id,
            self.full_name,
            self.owner.id,
            self.private,
            self.url,
        )
        .execute(&mut **transaction)
        .await?;

        sqlx::query!(
            "INSERT INTO github.app_installation_repositories (app_installation_id, repository_id)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING",
            app_installation_id,
            self.id,
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }
}
