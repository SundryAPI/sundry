use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use sqlx::{Decode, PgPool};

#[cfg(feature = "ssr")]
#[derive(thiserror::Error, Debug)]
pub enum ApplicationError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error("application name already exists for organization")]
    ApplicationNameInUse,
}

#[cfg_attr(feature = "ssr", derive(Decode))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Application {
    pub id: i64,
    pub organization_id: i64,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[cfg(feature = "ssr")]
impl Application {
    pub async fn create_with_name_organization_id(
        name: &str,
        organization_id: i64,
        pool: &PgPool,
    ) -> Result<Self, ApplicationError> {
        let application = sqlx::query_as!(
            Application,
            "INSERT INTO applications (organization_id, name) VALUES ($1, $2) RETURNING *",
            organization_id,
            name
        )
        .fetch_one(pool)
        .await;

        match application {
            Ok(application) => Ok(application),
            Err(e) => {
                if let Some(database_error) = e.as_database_error() {
                    if database_error.is_unique_violation() {
                        Err(ApplicationError::ApplicationNameInUse)
                    } else {
                        Err(ApplicationError::Sqlx(e))
                    }
                } else {
                    Err(ApplicationError::Sqlx(e))
                }
            }
        }
    }

    pub async fn get_with_id(
        application_id: i64,
        pool: &PgPool,
    ) -> Result<Option<Application>, ApplicationError> {
        Ok(sqlx::query_as!(
            Application,
            "SELECT * FROM applications WHERE id = $1 AND deleted_at IS NULL",
            application_id
        )
        .fetch_optional(pool)
        .await?)
    }

    pub async fn get_with_organization_id(
        organization_id: i64,
        pool: &PgPool,
    ) -> Result<Vec<Application>, ApplicationError> {
        Ok(sqlx::query_as!(
            Application,
            "SELECT * FROM applications WHERE organization_id = $1 AND deleted_at IS NULL",
            organization_id
        )
        .fetch_all(pool)
        .await?)
    }
}
