use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use sqlx::{Decode, PgPool};

#[cfg_attr(feature = "ssr", derive(Decode))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInstallation {
    pub id: i64,
    pub access_token: String,
    pub organization_id: i64,
    pub created_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[cfg(feature = "ssr")]
impl AppInstallation {
    pub async fn create(
        access_token: &str,
        organization_id: i64,
        pool: &PgPool,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            AppInstallation,
            r#"INSERT INTO slack.app_installations (access_token, organization_id)
            VALUES ($1, $2)
            ON CONFLICT (access_token) DO UPDATE SET organization_id = EXCLUDED.organization_id, deleted_at = NULL
            RETURNING id, access_token, organization_id, created_at, deleted_at"#,
            access_token,
            organization_id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(
            AppInstallation,
            r#"
            SELECT id, access_token, organization_id, created_at, deleted_at FROM slack.app_installations WHERE deleted_at is NULL
            "#,
        ).fetch_all(pool).await
    }

    pub async fn get_with_organization_id(
        organization_id: i64,
        pool: &PgPool,
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(
            AppInstallation,
            r#"
            SELECT id, access_token, organization_id, created_at, deleted_at FROM slack.app_installations WHERE deleted_at is NULL AND organization_id = $1
            "#,
            organization_id
        ).fetch_all(pool).await
    }

    pub async fn delete(self, pool: &PgPool) -> Result<u64, sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE slack.app_installations
            SET deleted_at = now()
            WHERE id = $1 AND deleted_at is NULL
            "#,
            self.id
        )
        .execute(pool)
        .await
        .map(|result| result.rows_affected())
    }
}
