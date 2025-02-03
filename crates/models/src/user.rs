use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHashString, PasswordHasher, PasswordVerifier,
        SaltString,
    },
    Argon2,
};
#[cfg(feature = "ssr")]
use sqlx::{Decode, PgPool};

#[cfg(feature = "ssr")]
use crate::organization::{Organization, OrganizationError, Role};

#[cfg_attr(feature = "ssr", derive(Decode))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub name: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[cfg(feature = "ssr")]
impl From<UserWithPassword> for User {
    fn from(user_with_password: UserWithPassword) -> Self {
        User {
            id: user_with_password.id,
            email: user_with_password.email,
            name: user_with_password.name,
            created_at: user_with_password.created_at,
            updated_at: user_with_password.updated_at,
        }
    }
}

#[cfg(feature = "ssr")]
#[derive(Debug, Clone, Decode, Serialize, Deserialize)]
pub struct UserWithPassword {
    pub id: i64,
    pub email: String,
    pub name: Option<String>,
    pub password_hash: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[cfg(feature = "ssr")]
#[derive(thiserror::Error, Debug)]
pub enum UserError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error("password hash: {0:?}")]
    PasswordHash(argon2::password_hash::Error),
    #[error("email: {0:?} already in use")]
    EmailInUse(String),
    #[error("parsing role: {0:?}")]
    ParsingRole(String),
}

#[cfg(feature = "ssr")]
impl From<OrganizationError> for UserError {
    fn from(error: OrganizationError) -> Self {
        match error {
            OrganizationError::Sqlx(e) => UserError::Sqlx(e),
            OrganizationError::PasswordHash(e) => UserError::PasswordHash(e),
            OrganizationError::ParsingRole(s) => UserError::ParsingRole(s),
        }
    }
}

#[cfg(feature = "ssr")]
impl User {
    pub async fn get_user_by_session_id(
        session_id: i64,
        pool: &PgPool,
    ) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
                SELECT users.id, users.email, users.name, users.created_at, users.updated_at
                FROM sessions
                INNER JOIN users on users.id = sessions.user_id
                WHERE sessions.id = $1 AND sessions.expires_at > CURRENT_TIMESTAMP AND sessions.logged_in
                ORDER BY sessions.expires_at DESC
                LIMIT 1"#,
            session_id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn get_user_by_email(
        email: &str,
        pool: &PgPool,
    ) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
                SELECT users.id, users.email, users.name, users.created_at, users.updated_at
                FROM users
                WHERE email = $1"#,
            email
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn create_with_email_and_password(
        email: &str,
        password: &str,
        name: Option<&str>,
        pool: &PgPool,
    ) -> Result<User, UserError> {
        let maybe_user_with_password: Option<UserWithPassword> = sqlx::query_as!(
            UserWithPassword,
            "SELECT users.id, users.email, users.name, users.created_at, users.updated_at, users.password_hash FROM users WHERE email = $1",
            email
        )
        .fetch_optional(pool).await?;

        // If the email is already in use exit
        // We may want to revisit this later and allow users to sign up with OAuth and
        // email and password
        if maybe_user_with_password
            .is_some_and(|user_with_password| user_with_password.password_hash.is_some())
        {
            return Err(UserError::EmailInUse(email.to_string()));
        }

        // The parsed_hash stores the salt
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(UserError::PasswordHash)?
            .to_string();
        let parsed_hash = PasswordHash::new(&password_hash)
            .map_err(UserError::PasswordHash)?
            .to_string();

        let mut transaction = pool.begin().await?;

        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (email, name, password_hash) VALUES ($1, $2, $3) RETURNING users.id, users.name, users.email, users.created_at, users.updated_at",
            email,
            name,
            parsed_hash
        )
        .fetch_one(&mut * transaction)
        .await?;

        let organization = Organization::create("Personal", &mut transaction).await?;
        organization
            .add_user(&user, Role::Owner, &mut transaction)
            .await?;

        transaction.commit().await?;

        Ok(user)
    }

    pub async fn create_with_github(
        github_account_id: i64,
        github_username: &str,
        users_name: Option<&str>,
        access_token: &str,
        email: &str,
        pool: &PgPool,
    ) -> Result<User, UserError> {
        let maybe_user = Self::get_user_by_email(email, pool).await?;

        let mut transaction = pool.begin().await?;

        // Get or create the user
        //
        // A user may exist if they have signed up with a previous oauth method
        // or with password and email
        let user = if let Some(user) = maybe_user {
            user
        } else {
            let user = sqlx::query_as!(
                User,
                "INSERT INTO users (email, name) VALUES ($1, $2) RETURNING users.id, users.email, users.name, users.created_at, users.updated_at",
                email,
                users_name
            )
            .fetch_one(&mut *transaction)
            .await?;

            let organization = Organization::create("Personal", &mut transaction).await?;
            organization
                .add_user(&user, Role::Owner, &mut transaction)
                .await?;

            user
        };

        user.add_github_oauth(
            github_account_id,
            github_username,
            access_token,
            &mut transaction,
        )
        .await?;

        transaction.commit().await?;

        Ok(user)
    }

    pub async fn add_github_oauth(
        &self,
        github_account_id: i64,
        github_username: &str,
        access_token: &str,
        conn: &mut sqlx::PgConnection,
    ) -> Result<(), UserError> {
        // We only care about storing their GitHub account id and username for now
        let provider_data =
            serde_json::json!({ "account_id": github_account_id, "login": github_username });
        let user_oauth_account_id: i64 = sqlx::query_scalar!(
            r#"
                INSERT INTO user_oauth_accounts (user_id, provider_id, provider_data)
                VALUES ($1, (SELECT id FROM oauth_providers WHERE name = 'github'), $2)
                ON CONFLICT (user_id, provider_id) DO UPDATE SET
                    provider_data = EXCLUDED.provider_data
                RETURNING id
            "#,
            self.id,
            provider_data
        )
        .fetch_one(&mut *conn)
        .await?;

        sqlx::query!(
            r#"INSERT INTO oauth_tokens (user_oauth_account_id, access_token)
            VALUES ($1, $2)"#,
            user_oauth_account_id,
            access_token
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    pub async fn add_slack_oauth(
        &self,
        access_token: &str,
        slack_id: &str,
        slack_email: &str,
        conn: &mut sqlx::PgConnection,
    ) -> Result<(), UserError> {
        let provider_data = serde_json::json!({ "user_id": slack_id, "email": slack_email });

        let user_oauth_account_id: i64 = sqlx::query_scalar!(
            r#"
                INSERT INTO user_oauth_accounts (user_id, provider_id, provider_data)
                VALUES ($1, (SELECT id FROM oauth_providers WHERE name = 'slack'), $2)
                ON CONFLICT (user_id, provider_id) DO UPDATE SET
                    provider_data = EXCLUDED.provider_data
                RETURNING id
            "#,
            self.id,
            provider_data
        )
        .fetch_one(&mut *conn)
        .await?;

        sqlx::query!(
            r#"INSERT INTO oauth_tokens (user_oauth_account_id, access_token)
            VALUES ($1, $2)"#,
            user_oauth_account_id,
            access_token
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    pub async fn get_with_email_password(
        email: &str,
        password: &str,
        pool: &PgPool,
    ) -> Result<Option<User>, UserError> {
        let maybe_user_with_password: Option<UserWithPassword> = sqlx::query_as!(
            UserWithPassword,
            "SELECT users.id, users.email, users.name, users.created_at, users.updated_at, users.password_hash FROM users WHERE email = $1",
            email
        )
        .fetch_optional(pool).await?;

        let user_with_password = if let Some(user_with_password) = maybe_user_with_password {
            user_with_password
        } else {
            return Ok(None);
        };

        // If they have no password_hash set return None
        // This most likely means they used an alternate method of signing in
        // Maybe we should tell them about it? It could be a security risk though
        if user_with_password.password_hash.is_none() {
            return Ok(None);
        }

        let password_hash: PasswordHashString =
            PasswordHashString::new(user_with_password.password_hash.as_ref().unwrap())
                .map_err(UserError::PasswordHash)?;
        let password_hash: PasswordHash = password_hash.password_hash();

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &password_hash)
            .ok()
            .map(|_| user_with_password.into()))
    }
}

#[cfg(feature = "ssr")]
pub async fn github_username_for_user_id(
    user_id: i64,
    pool: &PgPool,
) -> Result<Option<String>, sqlx::Error> {
    Ok(sqlx::query_scalar!(
        r##"
            SELECT provider_data->>'login' FROM user_oauth_accounts
            INNER JOIN oauth_providers ON user_oauth_accounts.provider_id = oauth_providers.id
            WHERE oauth_providers.name = 'github' AND user_id = $1
        "##,
        user_id
    )
    .fetch_optional(pool)
    .await?
    .flatten())
}

#[cfg(feature = "ssr")]
pub async fn slack_user_id_for_user_id(
    user_id: i64,
    pool: &PgPool,
) -> Result<Option<String>, sqlx::Error> {
    Ok(sqlx::query_scalar!(
        r##"
            SELECT provider_data->>'user_id' FROM user_oauth_accounts
            INNER JOIN oauth_providers ON user_oauth_accounts.provider_id = oauth_providers.id
            WHERE oauth_providers.name = 'slack' AND user_id = $1
        "##,
        user_id
    )
    .fetch_optional(pool)
    .await?
    .flatten())
}
