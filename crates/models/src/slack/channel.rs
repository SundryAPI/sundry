use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use sqlx::{Decode, Executor, Postgres};

#[cfg_attr(feature = "ssr", derive(Decode))]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub user: Option<String>,
    #[serde(default)]
    pub is_channel: bool,
    #[serde(default)]
    pub is_group: bool,
    #[serde(default)]
    pub is_im: bool,
    pub created: i64,
    #[serde(default)]
    pub creator: Option<String>,
    #[serde(default)]
    pub is_archived: bool,
    #[serde(default)]
    pub is_general: bool,
    #[serde(default)]
    pub unlinked: i32,
    #[serde(default)]
    pub name_normalized: Option<String>,
    #[serde(default)]
    pub is_shared: bool,
    #[serde(default)]
    pub is_ext_shared: bool,
    #[serde(default)]
    pub is_org_shared: bool,
    #[serde(default)]
    pub pending_shared: Vec<String>,
    #[serde(default)]
    pub is_pending_ext_shared: bool,
    #[serde(default)]
    pub is_member: bool,
    #[serde(default)]
    pub is_private: bool,
    #[serde(default)]
    pub is_mpim: bool,
    pub updated: i64,
    #[serde(default)]
    pub topic: Option<serde_json::Value>,
    #[serde(default)]
    pub purpose: Option<serde_json::Value>,
    #[serde(default)]
    pub previous_names: Vec<String>,
    #[serde(default)]
    pub num_members: i32,
}

#[cfg(feature = "ssr")]
impl Channel {
    pub async fn insert<'a, E>(&self, executor: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'a, Database = Postgres>,
    {
        sqlx::query!(
            r#"
            INSERT INTO slack.channels (
                id, name, "user", is_channel, is_group, is_im, created, creator, is_archived, is_general,
                unlinked, name_normalized, is_shared, is_ext_shared, is_org_shared, pending_shared,
                is_pending_ext_shared, is_member, is_private, is_mpim, updated, topic, purpose,
                previous_names, num_members
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25)
            ON CONFLICT DO NOTHING
            "#,
            self.id,
            self.name,
            self.user,
            self.is_channel,
            self.is_group,
            self.is_im,
            self.created,
            self.creator,
            self.is_archived,
            self.is_general,
            self.unlinked,
            self.name_normalized,
            self.is_shared,
            self.is_ext_shared,
            self.is_org_shared,
            &self.pending_shared,
            self.is_pending_ext_shared,
            self.is_member,
            self.is_private,
            self.is_mpim,
            self.updated,
            self.topic,
            self.purpose,
            &self.previous_names,
            self.num_members
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
        let names = data
            .iter()
            .map(|d| d.name.clone().unwrap_or_default())
            .collect::<Vec<_>>();
        let created = data.iter().map(|d| d.created).collect::<Vec<_>>();
        let creators = data
            .iter()
            .map(|d| d.creator.clone().unwrap_or_default())
            .collect::<Vec<_>>();
        let users = data
            .iter()
            .map(|d| d.user.clone().unwrap_or_default())
            .collect::<Vec<_>>();

        // id primary key
        // name TEXT,
        // created BIGINT NOT NULL,
        // creator TEXT, -- references slack.users(id),
        // "user" TEXT, -- references slack.users(id),

        sqlx::query!(
            r#"
            INSERT INTO slack.channels(id, name, created, creator, "user")
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::bigint[], $4::text[], $5::text[])
            ON CONFLICT DO NOTHING
            "#,
            &ids[..],
            &names[..],
            &created[..],
            &creators[..],
            &users[..],
        )
        .execute(e)
        .await?;

        Ok(())
    }

    pub async fn add_to_app<'a, E>(
        channel_id: &str,
        app_installation_id: i64,
        executor: E,
    ) -> Result<(), sqlx::Error>
    where
        E: Executor<'a, Database = Postgres>,
    {
        sqlx::query!(
            r#"
            INSERT INTO slack.app_installation_channels (app_installation_id, channel_id)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING
            "#,
            app_installation_id,
            channel_id,
        )
        .execute(executor)
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use sqlx::Row;

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn test_channel_user(pool: sqlx::PgPool) -> sqlx::Result<()> {
        let user = Some("sundry".to_string());
        let channel = Channel {
            id: "channel_id".into(),
            user: user.clone(),
            ..Default::default()
        };

        channel.insert(&pool).await?;

        let row = sqlx::query("SELECT * FROM slack.channels LIMIT 1")
            .fetch_one(&pool)
            .await?;

        assert_eq!(row.get::<Option<String>, _>("user"), user);

        Ok(())
    }
}
