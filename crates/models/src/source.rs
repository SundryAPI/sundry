use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[cfg(feature = "ssr")]
use sqlx::{Executor, PgPool, Postgres, Row, Transaction};

#[cfg(feature = "ssr")]
use crate::api::{application_key::ApplicationKey, user_key::UserKey};

pub type Sources = HashMap<Source, Vec<SourceItem>>;

#[cfg(feature = "ssr")]
pub async fn get_sources_for_user_key_and_application_key(
    user_key: &UserKey,
    _application_key: &ApplicationKey,
    pool: &sqlx::PgPool,
) -> Result<Sources, sqlx::Error> {
    get_sources_for_user_id_and_organization_id(user_key.user_id, user_key.organization_id, pool)
        .await
}

#[cfg(feature = "ssr")]
pub async fn get_sources_for_user_id_and_organization_id(
    _user_id: i64,
    organization_id: i64,
    pool: &sqlx::PgPool,
) -> Result<Sources, sqlx::Error> {
    let mut sources = HashMap::new();

    if !crate::github::app_installation::AppInstallation::get_with_organization_id(
        organization_id,
        pool,
    )
    .await?
    .is_empty()
    {
        sources.insert(
            Source::GitHub,
            vec![
                SourceItem::GitHub(GitHubItem::Issues),
                SourceItem::GitHub(GitHubItem::PullRequests),
                SourceItem::GitHub(GitHubItem::Repositories),
                SourceItem::GitHub(GitHubItem::Comments),
            ],
        );
    }

    if !crate::slack::app_installation::AppInstallation::get_with_organization_id(
        organization_id,
        pool,
    )
    .await?
    .is_empty()
    {
        sources.insert(
            Source::Slack,
            vec![
                SourceItem::Slack(SlackItem::Messages),
                SourceItem::Slack(SlackItem::Channels),
                SourceItem::Slack(SlackItem::Users),
            ],
        );
    }

    Ok(sources)
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
pub enum Source {
    GitHub,
    Google,
    Slack,
    Jira,
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Source::GitHub => write!(f, "GitHub"),
            Source::Google => write!(f, "Google"),
            Source::Slack => write!(f, "Slack"),
            Source::Jira => write!(f, "Jira"),
        }
    }
}

impl FromStr for Source {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GitHub" => Ok(Self::GitHub),
            "Google" => Ok(Self::Google),
            "Slack" => Ok(Self::Slack),
            s => Err(s.to_string()),
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
pub enum GoogleItem {
    Docs,
}

impl fmt::Display for GoogleItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GoogleItem::Docs => write!(f, "Docs"),
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
pub enum GitHubItem {
    Issues,
    PullRequests,
    Comments,
    Repositories,
}

impl fmt::Display for GitHubItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitHubItem::Issues => write!(f, "Issues"),
            GitHubItem::PullRequests => write!(f, "Pull Requests"),
            GitHubItem::Repositories => write!(f, "Repositories"),
            GitHubItem::Comments => write!(f, "Comments"),
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
pub enum SlackItem {
    Users,
    Messages,
    Channels,
    ChannelMembers,
}

impl fmt::Display for SlackItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SlackItem::Messages => write!(f, "Messages"),
            SlackItem::Channels => write!(f, "Channels"),
            SlackItem::Users => write!(f, "Users"),
            SlackItem::ChannelMembers => write!(f, "ChannelMembers"),
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
pub enum JiraItem {
    Sprints,
    Issues,
}

impl fmt::Display for JiraItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JiraItem::Sprints => write!(f, "Sprints"),
            JiraItem::Issues => write!(f, "Issues"),
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(untagged)]
#[serde(rename_all = "PascalCase")]
pub enum SourceItem {
    GitHub(GitHubItem),
    Slack(SlackItem),
    Jira(JiraItem),
    Google(GoogleItem),
}

impl SourceItem {
    pub fn source(&self) -> Source {
        match self {
            Self::GitHub(_) => Source::GitHub,
            Self::Google(_) => Source::Google,
            Self::Slack(_) => Source::Slack,
            Self::Jira(_) => Source::Jira,
        }
    }
}

impl From<GitHubItem> for SourceItem {
    fn from(value: GitHubItem) -> Self {
        Self::GitHub(value)
    }
}

impl From<SlackItem> for SourceItem {
    fn from(value: SlackItem) -> Self {
        Self::Slack(value)
    }
}

impl From<JiraItem> for SourceItem {
    fn from(value: JiraItem) -> Self {
        Self::Jira(value)
    }
}

impl fmt::Display for SourceItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SourceItem::GitHub(item) => write!(f, "GitHub: {}", item),
            SourceItem::Google(item) => write!(f, "Google: {}", item),
            SourceItem::Slack(item) => write!(f, "Slack: {}", item),
            SourceItem::Jira(item) => write!(f, "Jira: {}", item),
        }
    }
}

// TOOD: @kevin / @silas add the from str here for pulling from db
impl FromStr for SourceItem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GitHub: Issues" => Ok(Self::GitHub(GitHubItem::Issues)),
            "GitHub: Pull Requests" => Ok(Self::GitHub(GitHubItem::PullRequests)),
            "GitHub: Repositories" => Ok(Self::GitHub(GitHubItem::Repositories)),
            "GitHub: Comments" => Ok(Self::GitHub(GitHubItem::Comments)),
            "Slack: Messages" => Ok(Self::Slack(SlackItem::Messages)),
            "Slack: Users" => Ok(Self::Slack(SlackItem::Users)),
            "Slack: Channels" => Ok(Self::Slack(SlackItem::Channels)),
            "Slack: ChannelMembers" => Ok(Self::Slack(SlackItem::ChannelMembers)),
            s => Err(s.to_string()),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct RawData {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub organization_id: i64,
    pub data_id: String,
    pub source: Source,
    pub source_item: SourceItem,
    pub data: Value,
}

#[cfg(feature = "ssr")]
impl RawData {
    pub async fn create<S: Into<SourceItem>>(
        pool: &PgPool,
        organization_id: i64,
        data_id: String,
        source_item: S,
        data: Value,
    ) -> Result<(), sqlx::Error> {
        let source_item = source_item.into();
        sqlx::query!(
            r#"
            INSERT INTO raw (organization_id, data_id, source, source_item, data)
            VALUES ($1, $2, $3, $4, $5::jsonb)
            ON CONFLICT (data_id, source, source_item) DO UPDATE SET 
            data = EXCLUDED.data,
            deleted_at = NULL
            "#,
            organization_id,
            data_id,
            source_item.source().to_string(),
            source_item.to_string(),
            data,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn mark_as_normalized(
        id: i64,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO raw_normalized (raw_id, normalized_at)
                  VALUES ($1, CURRENT_TIMESTAMP)
                  ON CONFLICT (raw_id) DO NOTHING",
        )
        .bind(id)
        .execute(&mut **transaction)
        .await?;
        Ok(())
    }

    pub async fn mark_as_normalized_batch<'a, E>(e: E, ids: &[i64]) -> Result<(), sqlx::Error>
    where
        E: Executor<'a, Database = Postgres>,
    {
        sqlx::query!(
            r#"
            INSERT INTO raw_normalized(raw_id)
            SELECT * FROM UNNEST($1::bigint[])
            ON CONFLICT DO NOTHING
            "#,
            &ids[..],
        )
        .execute(e)
        .await?;

        Ok(())
    }

    pub async fn create_all<S: Into<SourceItem>>(
        pool: &PgPool,
        organization_id: i64,
        data_ids: &[String],
        source_item: S,
        data: &[Value],
    ) -> Result<(), sqlx::Error> {
        let source_item = source_item.into();
        let org_ids: Vec<_> = data.iter().map(|_| organization_id).collect();
        let sources: Vec<_> = data
            .iter()
            .map(|_| source_item.source().to_string())
            .collect();
        let source_items: Vec<_> = data.iter().map(|_| source_item.to_string()).collect();

        sqlx::query!(
            r#"
            INSERT INTO raw (organization_id, data_id, source, source_item, data)
            SELECT * FROM UNNEST($1::bigint[], $2::text[], $3::text[], $4::text[], $5::jsonb[])
            ON CONFLICT (data_id, source, source_item) DO UPDATE SET 
            data = EXCLUDED.data,
            deleted_at = NULL
            "#,
            &org_ids[..],
            data_ids,
            &sources[..],
            &source_items[..],
            data,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_by_source_item(
        pool: &PgPool,
        organization_id: i64,
        source_item: SourceItem,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let records = sqlx::query!(
            "SELECT * FROM raw WHERE organization_id = $1 AND source_item = $2",
            organization_id,
            source_item.to_string(),
        )
        .fetch_all(pool)
        .await?;

        Ok(records
            .into_iter()
            .map(|record| Self {
                id: record.id,
                created_at: record.created_at,
                deleted_at: record.deleted_at,
                organization_id,
                data_id: record.data_id,
                source: Source::from_str(&record.source).unwrap(),
                source_item: SourceItem::from_str(&record.source_item).unwrap(),
                data: record.data,
            })
            .collect())
    }

    pub async fn get_slack_unnormalized(
        pool: &PgPool,
        organization_id: i64,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let records = sqlx::query!(
            "SELECT id, created_at, deleted_at, data_id, source_item, data, source
        FROM raw
        LEFT JOIN raw_normalized rn
        ON raw.id = rn.raw_id
        WHERE rn.normalized_at IS NULL AND organization_id = $1 AND source = 'Slack'
        ORDER BY
            CASE
                WHEN source_item = 'Slack: Users' THEN 1
                WHEN source_item = 'Slack: Channels' THEN 2
                WHEN source_item = 'Slack: ChannelMembers' THEN 3
                WHEN source_item = 'Slack: Messages' THEN 4
                ELSE 5 -- for any other source_items, if they exist
            END,
            id LIMIT 10000;
        ",
            organization_id,
        )
        .fetch_all(pool)
        .await?;

        Ok(records
            .into_iter()
            .map(|record| Self {
                id: record.id,
                created_at: record.created_at,
                deleted_at: record.deleted_at,
                organization_id,
                data_id: record.data_id,
                source: Source::from_str(&record.source).unwrap(),
                source_item: SourceItem::from_str(&record.source_item).unwrap(),
                data: record.data,
            })
            .collect())
    }

    pub async fn get_github_unnormalized(
        pool: &PgPool,
        organization_id: i64,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let records = sqlx::query!(
        "SELECT id, created_at, deleted_at, data_id, data, source_item, source FROM raw LEFT JOIN raw_normalized rn ON raw.id = rn.raw_id WHERE rn.normalized_at IS NULL AND organization_id = $1 AND source = 'GitHub' LIMIT 10000",
            organization_id,
        )
        .fetch_all(pool)
        .await?;

        Ok(records
            .into_iter()
            .map(|record| Self {
                id: record.id,
                created_at: record.created_at,
                deleted_at: record.deleted_at,
                organization_id,
                data_id: record.data_id,
                source: Source::from_str(&record.source).unwrap(),
                source_item: SourceItem::from_str(&record.source_item).unwrap(),
                data: record.data,
            })
            .collect())
    }

    pub async fn get_github_comments_urls(
        pool: &PgPool,
        organization_id: i64,
    ) -> Result<Vec<String>, sqlx::Error> {
        let records = sqlx::query!(
            "SELECT COALESCE((data->>'comments_url'),(data->>'url')) AS comments_url FROM raw WHERE organization_id = $1 AND (source_item = $2 OR source_item = $3)",
            organization_id,
            SourceItem::GitHub(GitHubItem::Issues).to_string(),
            SourceItem::GitHub(GitHubItem::PullRequests).to_string(),
        ).fetch_all(pool).await?;

        Ok(records
            .into_iter()
            .filter_map(|record| record.comments_url)
            .collect())
    }
}

#[cfg(feature = "ssr")]
impl From<&sqlx::postgres::PgRow> for RawData {
    fn from(row: &sqlx::postgres::PgRow) -> Self {
        let source = Source::from_str(row.get("source")).unwrap();
        let source_item = SourceItem::from_str(row.get("source_item")).unwrap();

        Self {
            id: row.get("organization_id"),
            created_at: row.get("created_at"),
            deleted_at: row.get("deleted_at"),
            organization_id: row.get("organization_id"),
            data_id: row.get("data_id"),
            source,
            source_item,
            data: row.get("data"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::organization::Organization;

    use super::*;

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn test_get_github_unnormalized(pool: PgPool) -> sqlx::Result<()> {
        let mut conn = pool.acquire().await?;
        let org = Organization::create("test", &mut conn).await.unwrap();
        sqlx::query!(
            r#"
            INSERT INTO raw (data, source_item, organization_id, source, data_id) 
            VALUES ('"test_data"'::jsonb, 'GitHub: Repositories', 1, 'GitHub', '1')
        "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        let data = RawData::get_github_unnormalized(&pool, org.id).await?;
        assert_eq!(data.len(), 1);

        // normalize the data
        for item in data {
            let mut transaction = pool.begin().await.unwrap();
            RawData::mark_as_normalized(item.id, &mut transaction).await?;
            transaction.commit().await.unwrap();
        }

        let data = RawData::get_github_unnormalized(&pool, org.id).await?;
        assert!(data.is_empty());
        Ok(())
    }

    #[test]
    fn test_source_string_conversions() {
        let sources = [Source::GitHub, Source::Slack];
        for source in sources {
            assert_eq!(Source::from_str(&source.to_string()).unwrap(), source);
        }
    }

    #[test]
    fn test_source_item_string_conversions() {
        let source_items = [
            SourceItem::GitHub(GitHubItem::Issues),
            SourceItem::GitHub(GitHubItem::PullRequests),
            SourceItem::GitHub(GitHubItem::Comments),
            SourceItem::GitHub(GitHubItem::Repositories),
            SourceItem::Slack(SlackItem::Messages),
            SourceItem::Slack(SlackItem::Channels),
            SourceItem::Slack(SlackItem::Users),
            SourceItem::Slack(SlackItem::ChannelMembers),
        ];
        for source_item in source_items {
            assert_eq!(
                SourceItem::from_str(&source_item.to_string()).unwrap(),
                source_item
            );
        }
    }
}
