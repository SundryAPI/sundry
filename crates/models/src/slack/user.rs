use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use sqlx::{Decode, Executor, Postgres};

#[cfg_attr(feature = "ssr", derive(Decode))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub team_id: String,
    pub name: String,
    #[serde(default)]
    pub real_name: Option<String>,
}

#[cfg(feature = "ssr")]
impl User {
    pub async fn insert<'a, E>(&self, executor: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'a, Database = Postgres>,
    {
        sqlx::query!(
            r#"
            INSERT INTO slack.users (
                id, team_id, name, real_name
            ) VALUES ($1, $2, $3, $4)
            ON CONFLICT DO NOTHING
            "#,
            self.id,
            self.team_id,
            self.name,
            self.real_name,
        )
        .execute(executor)
        .await?;
        Ok(())
    }

    pub async fn insert_batch<'a, E>(e: E, data: &[Self]) -> Result<(), sqlx::Error>
    where
        E: Executor<'a, Database = Postgres>,
    {
        let ids = data.iter().map(|d| d.id.clone()).collect::<Vec<_>>();
        let team_ids = data.iter().map(|d| d.team_id.clone()).collect::<Vec<_>>();
        let names = data.iter().map(|d| d.name.clone()).collect::<Vec<_>>();
        let real_names = data
            .iter()
            .map(|d| d.real_name.clone().unwrap_or_default())
            .collect::<Vec<_>>();

        sqlx::query!(
            r#"
            INSERT INTO slack.users(id, team_id, name, real_name)
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::text[])
            ON CONFLICT DO NOTHING
            "#,
            &ids[..],
            &team_ids[..],
            &names[..],
            &real_names[..],
        )
        .execute(e)
        .await?;

        Ok(())
    }

    pub async fn add_to_channel<'a, E>(
        channel_id: String,
        user_id: String,
        executor: E,
    ) -> Result<(), sqlx::Error>
    where
        E: Executor<'a, Database = Postgres>,
    {
        sqlx::query!(
            r#"
            INSERT INTO slack.channel_members (channel_id, user_id)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING
            "#,
            channel_id,
            user_id
        )
        .execute(executor)
        .await?;
        Ok(())
    }

    pub async fn add_to_channel_batch<'a, E>(
        e: E,
        channel_ids: &[String],
        member_ids: &[String],
    ) -> Result<(), sqlx::Error>
    where
        E: Executor<'a, Database = Postgres>,
    {
        sqlx::query!(
            r#"
            INSERT INTO slack.channel_members(channel_id, user_id)
            SELECT * FROM UNNEST($1::text[], $2::text[])
            ON CONFLICT DO NOTHING
            "#,
            &channel_ids[..],
            &member_ids[..],
        )
        .execute(e)
        .await?;

        Ok(())
    }
}
