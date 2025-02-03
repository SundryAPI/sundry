use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use sqlx::{Decode, PgPool};

#[cfg(feature = "ssr")]
#[derive(thiserror::Error, Debug)]
pub enum ApplicationKeyError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error("password hash: {0:?}")]
    PasswordHash(argon2::password_hash::Error),
}

#[cfg_attr(feature = "ssr", derive(Decode))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationKey {
    pub id: i64,
    pub application_id: i64,
    pub description: String,
    pub is_admin: bool,
    pub key_id: String,
    pub key_hash: String,
    pub created_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationKeyWithKey {
    pub id: i64,
    pub application_id: i64,
    pub description: String,
    pub is_admin: bool,
    pub key_id: String,
    pub key: String,
    pub key_hash: String,
    pub created_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[cfg(feature = "ssr")]
impl ApplicationKeyWithKey {
    pub fn from_key_and_string(other_key: ApplicationKey, key: String) -> Self {
        ApplicationKeyWithKey {
            id: other_key.id,
            application_id: other_key.application_id,
            description: other_key.description,
            is_admin: other_key.is_admin,
            key_id: other_key.key_id,
            key,
            key_hash: other_key.key_hash,
            created_at: other_key.created_at,
            deleted_at: other_key.deleted_at,
        }
    }

    pub async fn create_with_application_id(
        application_id: i64,
        description: &str,
        pool: &PgPool,
    ) -> Result<ApplicationKeyWithKey, ApplicationKeyError> {
        use crate::api::utils::generate_api_key;

        let (key_id, key, key_hash) =
            generate_api_key().map_err(ApplicationKeyError::PasswordHash)?;

        let user_key: ApplicationKey = sqlx::query_as!(ApplicationKey, "INSERT INTO application_api_keys (application_id, key_id, key_hash, description) VALUES ($1, $2, $3, $4) RETURNING *", application_id, key_id, key_hash, description).fetch_one(pool).await?;

        Ok(ApplicationKeyWithKey::from_key_and_string(user_key, key))
    }
}

#[cfg(feature = "ssr")]
impl ApplicationKey {
    pub async fn get_with_application_id(
        application_id: i64,
        pool: &PgPool,
    ) -> Result<Vec<ApplicationKey>, ApplicationKeyError> {
        Ok(sqlx::query_as!(
            ApplicationKey,
            "SELECT * FROM application_api_keys WHERE application_id = $1 AND deleted_at IS NULL",
            application_id
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn get_with_key_id_key(
        key_id: &str,
        key: &str,
        pool: &PgPool,
    ) -> Result<Option<ApplicationKey>, ApplicationKeyError> {
        use argon2::{password_hash::PasswordHashString, Argon2, PasswordHash, PasswordVerifier};

        let maybe_application_key = sqlx::query_as!(
            ApplicationKey,
            "SELECT * FROM application_api_keys WHERE key_id = $1",
            key_id
        )
        .fetch_optional(pool)
        .await?;

        if let Some(application_key) = maybe_application_key {
            let key_hash: PasswordHashString = PasswordHashString::new(&application_key.key_hash)
                .map_err(ApplicationKeyError::PasswordHash)?;
            let key_hash: PasswordHash = key_hash.password_hash();

            Ok(Argon2::default()
                .verify_password(key.as_bytes(), &key_hash)
                .ok()
                .map(|_| application_key))
        } else {
            Ok(None)
        }
    }

    pub async fn get_with_key_id(
        key_id: &str,
        pool: &PgPool,
    ) -> Result<Option<ApplicationKey>, ApplicationKeyError> {
        Ok(sqlx::query_as!(
            ApplicationKey,
            "SELECT * FROM application_api_keys WHERE key_id = $1",
            key_id
        )
        .fetch_optional(pool)
        .await?)
    }

    pub async fn delete(self, pool: &PgPool) -> Result<(), ApplicationKeyError> {
        sqlx::query!(
            "UPDATE application_api_keys SET deleted_at = NOW() WHERE id = $1",
            self.id
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
