use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use sqlx::{Decode, PgPool};

#[cfg(feature = "ssr")]
#[derive(thiserror::Error, Debug)]
pub enum UserKeyError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error("password hash: {0:?}")]
    PasswordHash(argon2::password_hash::Error),
}

#[cfg_attr(feature = "ssr", derive(Decode))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserKey {
    pub id: i64,
    pub user_id: i64,
    pub organization_id: i64,
    pub is_admin: bool,
    pub description: String,
    pub key_id: String,
    pub key_hash: String,
    pub created_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserKeyWithKey {
    pub id: i64,
    pub user_id: i64,
    pub organization_id: i64,
    pub is_admin: bool,
    pub description: String,
    pub key_id: String,
    pub key: String,
    pub key_hash: String,
    pub created_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[cfg(feature = "ssr")]
impl UserKeyWithKey {
    pub fn from_key_and_string(user_key: UserKey, key: String) -> Self {
        UserKeyWithKey {
            id: user_key.id,
            user_id: user_key.user_id,
            organization_id: user_key.organization_id,
            is_admin: user_key.is_admin,
            description: user_key.description,
            key_id: user_key.key_id,
            key,
            key_hash: user_key.key_hash,
            created_at: user_key.created_at,
            deleted_at: user_key.deleted_at,
        }
    }

    pub async fn create_with_user_id_organization_id(
        user_id: i64,
        organization_id: i64,
        description: &str,
        is_admin: bool,
        conn: &mut sqlx::PgConnection,
    ) -> Result<UserKeyWithKey, UserKeyError> {
        use crate::api::utils::generate_api_key;
        let (key_id, key, key_hash) = generate_api_key().map_err(UserKeyError::PasswordHash)?;
        let user_key: UserKey = sqlx::query_as!(UserKey, "INSERT INTO user_api_keys (user_id, organization_id, is_admin, description, key_id, key_hash) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *", user_id, organization_id, is_admin, description, key_id, key_hash).fetch_one(&mut *conn).await?;
        Ok(UserKeyWithKey::from_key_and_string(user_key, key))
    }
}

#[cfg(feature = "ssr")]
impl UserKey {
    pub async fn get_admin_key(
        user_id: i64,
        organization_id: i64,
        pool: &PgPool,
    ) -> Result<Option<UserKey>, UserKeyError> {
        Ok(sqlx::query_as!(UserKey, "SELECT * FROM user_api_keys WHERE user_id = $1 AND organization_id = $2 AND is_admin = TRUE AND deleted_at IS NULL", user_id, organization_id).fetch_optional(pool).await?)
    }

    pub async fn get_with_user_id_organization_id(
        user_id: i64,
        organization_id: i64,
        pool: &PgPool,
    ) -> Result<Vec<UserKey>, UserKeyError> {
        Ok(sqlx::query_as!(UserKey, "SELECT * FROM user_api_keys WHERE user_id = $1 AND organization_id = $2 AND deleted_at IS NULL AND is_admin = FALSE", user_id, organization_id).fetch_all(pool).await?)
    }

    pub async fn get_with_key_id_key(
        key_id: &str,
        key: &str,
        pool: &PgPool,
    ) -> Result<Option<UserKey>, UserKeyError> {
        use argon2::{password_hash::PasswordHashString, Argon2, PasswordHash, PasswordVerifier};

        let maybe_user_key = sqlx::query_as!(
            UserKey,
            "SELECT * FROM user_api_keys WHERE key_id = $1",
            key_id
        )
        .fetch_optional(pool)
        .await?;

        if let Some(user_key) = maybe_user_key {
            let key_hash: PasswordHashString =
                PasswordHashString::new(&user_key.key_hash).map_err(UserKeyError::PasswordHash)?;
            let key_hash: PasswordHash = key_hash.password_hash();

            Ok(Argon2::default()
                .verify_password(key.as_bytes(), &key_hash)
                .ok()
                .map(|_| user_key))
        } else {
            Ok(None)
        }
    }

    pub async fn get_with_key_id(
        key_id: &str,
        pool: &PgPool,
    ) -> Result<Option<UserKey>, UserKeyError> {
        Ok(sqlx::query_as!(
            UserKey,
            "SELECT * FROM user_api_keys WHERE key_id = $1",
            key_id
        )
        .fetch_optional(pool)
        .await?)
    }

    pub async fn delete(self, pool: &PgPool) -> Result<(), UserKeyError> {
        sqlx::query!(
            "UPDATE user_api_keys SET deleted_at = NOW() WHERE id = $1",
            self.id
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
