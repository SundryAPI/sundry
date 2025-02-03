use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use sqlx::{Decode, Executor, PgPool, Postgres};

#[cfg_attr(feature = "ssr", derive(Decode))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInstallation {
    pub id: i64,
    pub installation_id: i64,
    pub organization_id: i64,
    pub created_at: NaiveDateTime,
}

#[cfg_attr(feature = "ssr", derive(Decode))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInstallationMaybeWithoutOrganization {
    pub id: i64,
    pub installation_id: i64,
    pub organization_id: Option<i64>,
    pub created_at: NaiveDateTime,
}

impl From<AppInstallationMaybeWithoutOrganization> for AppInstallation {
    fn from(maybe: AppInstallationMaybeWithoutOrganization) -> Self {
        Self {
            id: maybe.id,
            installation_id: maybe.installation_id,
            organization_id: maybe
                .organization_id
                .expect("AppInstallation must have an organization_id"),
            created_at: maybe.created_at,
        }
    }
}

#[cfg(feature = "ssr")]
impl AppInstallationMaybeWithoutOrganization {
    pub async fn create<'a, E>(installation_id: i64, executor: E) -> Result<Self, sqlx::Error>
    where
        E: Executor<'a, Database = Postgres>,
    {
        sqlx::query_as!(
            Self,
            r#"INSERT INTO github.app_installations (installation_id)
            VALUES ($1)
            ON CONFLICT (installation_id) DO NOTHING
            RETURNING id, installation_id, organization_id, created_at"#,
            installation_id
        )
        .fetch_one(executor)
        .await
    }
}

#[cfg(feature = "ssr")]
impl AppInstallation {
    pub async fn create(
        installation_id: i64,
        organization_id: i64,
        pool: &PgPool,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            AppInstallationMaybeWithoutOrganization,
            r#"INSERT INTO github.app_installations (installation_id, organization_id)
            VALUES ($1, $2)
            ON CONFLICT (installation_id) DO UPDATE SET organization_id = EXCLUDED.organization_id
            RETURNING id, installation_id, organization_id, created_at"#,
            installation_id,
            organization_id
        )
        .fetch_one(pool)
        .await
        .map(|installation| installation.into())
    }

    pub async fn get_with_organization_id(
        organization_id: i64,
        pool: &PgPool,
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(
            AppInstallationMaybeWithoutOrganization,
            r#"
            SELECT id, installation_id, organization_id, created_at FROM github.app_installations WHERE organization_id = $1 AND deleted_at is NULL
            "#,
            organization_id
        ).fetch_all(pool).await.map(|installation| installation.into_iter().map(|x| x.into()).collect())
    }

    pub async fn get_with_installation_id(
        installation_id: i64,
        pool: &PgPool,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            AppInstallationMaybeWithoutOrganization,
            r#"
            SELECT id, installation_id, organization_id, created_at FROM github.app_installations WHERE installation_id = $1 AND deleted_at is NULL
            "#,
            installation_id
        ).fetch_one(pool).await.map(|installation| installation.into())
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(
            AppInstallationMaybeWithoutOrganization,
            r#"
            SELECT id, installation_id, organization_id, created_at FROM github.app_installations WHERE deleted_at is NULL
            "#,
        ).fetch_all(pool).await.map(|installation| installation.into_iter().map(|x| x.into()).collect())
    }

    pub async fn delete(self, pool: &PgPool) -> Result<u64, sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE github.app_installations
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
