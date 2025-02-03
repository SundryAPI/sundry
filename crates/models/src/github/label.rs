use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use sqlx::{Decode, Executor, Postgres};

#[cfg_attr(feature = "ssr", derive(Decode))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

#[cfg(feature = "ssr")]
impl Label {
    pub async fn create<'a, E>(
        id: i64,
        name: String,
        description: Option<String>,
        executor: E,
    ) -> Result<Self, sqlx::Error>
    where
        E: Executor<'a, Database = Postgres>,
    {
        sqlx::query_as!(
            Self,
            "INSERT INTO github.labels (id, name, description)
            VALUES ($1, $2, $3)
            ON CONFLICT DO NOTHING
            RETURNING id, name, description",
            id,
            name,
            description
        )
        .fetch_one(executor)
        .await
    }

    pub async fn insert<'a, E>(&self, executor: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'a, Database = Postgres>,
    {
        sqlx::query!(
            "INSERT INTO github.labels (id, name, description)
            VALUES ($1, $2, $3)
            ON CONFLICT DO NOTHING",
            self.id,
            self.name,
            self.description
        )
        .execute(executor)
        .await?;

        Ok(())
    }
}
