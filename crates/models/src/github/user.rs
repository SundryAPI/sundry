use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use sqlx::{Decode, Executor, Postgres};

#[cfg_attr(feature = "ssr", derive(Decode))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub login: String,
}

#[cfg(feature = "ssr")]
impl User {
    pub async fn create<'a, E>(id: i64, login: String, executor: E) -> Result<Self, sqlx::Error>
    where
        E: Executor<'a, Database = Postgres>,
    {
        sqlx::query_as!(
            Self,
            "INSERT INTO github.users (id, login)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING
            RETURNING id, login",
            id,
            login,
        )
        .fetch_one(executor)
        .await
    }

    pub async fn insert<'a, E>(&self, executor: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'a, Database = Postgres>,
    {
        sqlx::query_as!(
            Self,
            "INSERT INTO github.users (id, login)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING",
            self.id,
            self.login,
        )
        .execute(executor)
        .await?;

        Ok(())
    }
}
