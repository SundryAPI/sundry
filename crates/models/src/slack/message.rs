use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use sqlx::{Decode, Executor, Postgres};

#[cfg_attr(feature = "ssr", derive(Decode))]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Message {
    pub channel_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "user")]
    pub r#user: Option<String>,
    pub text: Option<String>,
    pub thread_ts: Option<String>,
    pub parent_user_id: Option<String>,
    pub ts: String,
}

#[cfg(feature = "ssr")]
impl Message {
    pub async fn insert<'a, E>(&self, executor: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'a, Database = Postgres>,
    {
        sqlx::query!(
            r#"
            INSERT INTO slack.messages (
                channel_id, "type", "user", text, thread_ts, parent_user_id, ts
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT DO NOTHING
            "#,
            &self.channel_id,
            &self.r#type,
            self.r#user,
            self.text,
            self.thread_ts,
            self.parent_user_id,
            &self.ts
        )
        .execute(executor)
        .await?;

        Ok(())
    }

    pub async fn insert_batch<'a, E>(e: E, data: &[Self]) -> Result<(), sqlx::Error>
    where
        E: Executor<'a, Database = Postgres>,
    {
        let channel_ids = data
            .iter()
            .map(|d| d.channel_id.clone())
            .collect::<Vec<_>>();
        let types = data.iter().map(|d| d.r#type.clone()).collect::<Vec<_>>();
        let users = data
            .iter()
            .map(|d| d.user.clone().unwrap_or_default())
            .collect::<Vec<_>>();
        let texts = data
            .iter()
            .map(|d| d.text.clone().unwrap_or_default())
            .collect::<Vec<_>>();
        let thread_ts = data
            .iter()
            .map(|d| d.thread_ts.clone().unwrap_or_default())
            .collect::<Vec<_>>();
        let parent_user_ids = data
            .iter()
            .map(|d| d.parent_user_id.clone().unwrap_or_default())
            .collect::<Vec<_>>();
        let ts = data.iter().map(|d| d.ts.clone()).collect::<Vec<_>>();

        sqlx::query!(
            r#"
            INSERT INTO slack.messages(channel_id, "type", "user", text, thread_ts, parent_user_id, ts)
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::text[], $5::text[], $6::text[], $7::text[])
            ON CONFLICT DO NOTHING
            "#,
            &channel_ids[..],
            &types[..],
            &users[..],
            &texts[..],
            &thread_ts[..],
            &parent_user_ids[..],
            &ts[..],
        )
        .execute(e)
        .await?;

        Ok(())
    }

    pub async fn latest<'a, E>(channel_id: &str, executor: E) -> Result<Option<String>, sqlx::Error>
    where
        E: Executor<'a, Database = Postgres>,
    {
        let record = sqlx::query!(
            r#"
            SELECT ts FROM slack.messages WHERE channel_id = $1 ORDER BY ts DESC LIMIT 1
            "#,
            channel_id,
        )
        .fetch_optional(executor)
        .await?;

        Ok(record.map(|r| r.ts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn test_message_latest(pool: sqlx::PgPool) -> sqlx::Result<()> {
        let channel_id = "channel".to_string();
        let r#type = "message".to_string();
        let ts1 = "1".to_string();
        let ts2 = "2".to_string();

        let message_1 = Message {
            channel_id: channel_id.clone(),
            r#type: r#type.clone(),
            ts: ts1,
            ..Default::default()
        };
        let message_2 = Message {
            channel_id: channel_id.clone(),
            r#type,
            ts: ts2.clone(),
            ..Default::default()
        };

        message_1.insert(&pool).await?;
        message_2.insert(&pool).await?;

        let latest = Message::latest(&channel_id, &pool).await?;
        assert_eq!(latest, Some(ts2));

        Ok(())
    }
}
