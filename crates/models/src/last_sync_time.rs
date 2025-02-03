use std::str::FromStr;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::{PgPool, Result, Row};

use crate::source::{Source, SourceItem};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct LastSyncTime {
    id: i64,
    created_at: NaiveDateTime,
    deleted_at: Option<NaiveDateTime>,
    organization_id: i64,
    source: Source,
    source_item: SourceItem,
    last_sync_time: Option<NaiveDateTime>,
}

#[cfg(feature = "ssr")]
impl From<&sqlx::postgres::PgRow> for LastSyncTime {
    fn from(row: &sqlx::postgres::PgRow) -> Self {
        let source = Source::from_str(row.get("source")).unwrap();
        let source_item = SourceItem::from_str(row.get("source_item")).unwrap();

        Self {
            id: row.get("id"),
            created_at: row.get("created_at"),
            deleted_at: row.get("deleted_at"),
            organization_id: row.get("organization_id"),
            source,
            source_item,
            last_sync_time: row.get("last_sync_time"),
        }
    }
}

#[cfg(feature = "ssr")]
impl LastSyncTime {
    pub async fn get<S: Into<SourceItem>>(
        pool: &PgPool,
        organization_id: i64,
        source_item: S,
    ) -> Result<Option<NaiveDateTime>> {
        let source_item = source_item.into();
        let row = sqlx::query!(
            r#"
            SELECT last_sync_time
            FROM last_sync_time
            WHERE organization_id = $1 AND source = $2 AND source_item = $3 AND deleted_at IS NULL
            "#,
            organization_id,
            source_item.source().to_string(),
            source_item.to_string()
        )
        .fetch_optional(pool)
        .await?;

        Ok(row.and_then(|r| r.last_sync_time))
    }

    pub async fn update<S: Into<SourceItem>>(
        pool: &PgPool,
        organization_id: i64,
        source_item: S,
    ) -> Result<()> {
        let source_item = source_item.into();
        let result = sqlx::query!(
            r#"
            INSERT INTO last_sync_time (organization_id, source, source_item, last_sync_time)
            VALUES ($1, $2, $3, NULL)
            ON CONFLICT (organization_id, source, source_item)
            DO UPDATE SET last_sync_time = CURRENT_TIMESTAMP, deleted_at = NULL
            "#,
            organization_id,
            source_item.source().to_string(),
            source_item.to_string(),
        )
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }
}
