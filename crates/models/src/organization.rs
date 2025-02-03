use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::fmt;

#[cfg(feature = "ssr")]
use crate::api::user_key::{UserKeyError, UserKeyWithKey};

#[cfg(feature = "ssr")]
use sqlx::Decode;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Role {
    Admin = 0,
    Owner = 1,
}

impl TryFrom<String> for Role {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "admin" => Ok(Role::Admin),
            "owner" => Ok(Role::Owner),
            _ => Err(format!("No role found for '{}'", s)),
        }
    }
}

#[cfg(feature = "ssr")]
#[derive(thiserror::Error, Debug)]
pub enum OrganizationError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error("parsing role: {0:?}")]
    ParsingRole(String),
    #[error("password hash: {0:?}")]
    PasswordHash(argon2::password_hash::Error),
}

#[cfg(feature = "ssr")]
impl From<UserKeyError> for OrganizationError {
    fn from(error: UserKeyError) -> Self {
        match error {
            UserKeyError::Sqlx(e) => OrganizationError::Sqlx(e),
            UserKeyError::PasswordHash(e) => OrganizationError::PasswordHash(e),
        }
    }
}

#[cfg(feature = "ssr")]
impl Role {
    pub async fn get_with_user_id_and_organization_id(
        user_id: i64,
        organization_id: i64,
        pool: &sqlx::PgPool,
    ) -> Result<Option<Self>, OrganizationError> {
        let role_str: Option<String> = sqlx::query_scalar!(
            r#"
            SELECT user_organizations.role
            FROM user_organizations
            INNER JOIN users ON users.id = user_organizations.user_id
            WHERE users.id = $1 AND user_organizations.organization_id = $2
            "#,
            user_id,
            organization_id
        )
        .fetch_optional(pool)
        .await?;

        role_str
            .map(|role_str| Role::try_from(role_str).map_err(OrganizationError::ParsingRole))
            .transpose()
    }

    pub async fn get_with_session_and_organization(
        session_id: i64,
        organization_id: i64,
        pool: &sqlx::PgPool,
    ) -> Result<Option<Self>, OrganizationError> {
        let role_str: Option<String> = sqlx::query_scalar!(
            r#"
            SELECT user_organizations.role
            FROM user_organizations
            INNER JOIN sessions ON sessions.user_id = user_organizations.user_id
            WHERE sessions.id = $1 AND user_organizations.organization_id = $2
            "#,
            session_id,
            organization_id
        )
        .fetch_optional(pool)
        .await?;

        role_str
            .map(|role_str| Role::try_from(role_str).map_err(OrganizationError::ParsingRole))
            .transpose()
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Owner => write!(f, "owner"),
            Role::Admin => write!(f, "admin"),
        }
    }
}

#[cfg_attr(feature = "ssr", derive(Decode))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Organization {
    pub id: i64,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct OrganizationUser {
    pub id: i64,
    pub email: String,
    pub role: Role,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[cfg(feature = "ssr")]
impl Organization {
    pub async fn create(
        name: &str,
        conn: &mut sqlx::PgConnection,
    ) -> Result<Self, OrganizationError> {
        Ok(sqlx::query_as!(Self, "INSERT INTO organizations (name) VALUES ($1) RETURNING id, name, created_at, updated_at", name).fetch_one(&mut *conn).await?)
    }

    pub async fn add_user(
        &self,
        user: &crate::user::User,
        role: Role,
        conn: &mut sqlx::PgConnection,
    ) -> Result<(), OrganizationError> {
        sqlx::query!(
            "INSERT INTO user_organizations (user_id, organization_id, role) VALUES ($1, $2, $3)",
            user.id,
            self.id,
            role.to_string()
        )
        .execute(&mut *conn)
        .await?;

        // Add a "admin" key to be used by Sundry admins
        UserKeyWithKey::create_with_user_id_organization_id(user.id, self.id, "", true, conn)
            .await?;

        Ok(())
    }

    pub async fn get_with_user(
        user_id: i64,
        pool: &sqlx::PgPool,
    ) -> Result<Vec<Self>, OrganizationError> {
        Ok(sqlx::query_as!(Self,
            r#"SELECT organizations.id, organizations.name, organizations.created_at, organizations.updated_at
            FROM organizations INNER JOIN user_organizations
            ON user_organizations.organization_id = organizations.id
            WHERE user_organizations.user_id = $1
            "#, user_id).fetch_all(pool).await?)
    }

    pub async fn get_with_id_and_user(
        id: i64,
        user_id: i64,
        pool: &sqlx::PgPool,
    ) -> Result<Option<Self>, OrganizationError> {
        Ok(sqlx::query_as!(Self,
            r#"SELECT organizations.id, organizations.name, organizations.created_at, organizations.updated_at
            FROM organizations INNER JOIN user_organizations
            ON user_organizations.organization_id = organizations.id
            WHERE id = $1 AND user_organizations.user_id = $2
            "#, id, user_id).fetch_optional(pool).await?)
    }

    pub async fn user_is_owner(
        &self,
        user_id: i64,
        pool: &sqlx::PgPool,
    ) -> Result<bool, OrganizationError> {
        let role: Option<String> = sqlx::query_scalar!(
            r#"
            SELECT role
            FROM user_organizations
            WHERE user_id = $1 AND organization_id = $2
            "#,
            user_id,
            self.id
        )
        .fetch_optional(pool)
        .await?;

        Ok(role == Some("owner".to_string()))
    }

    pub async fn get_with_id(
        id: i64,
        pool: &sqlx::PgPool,
    ) -> Result<Option<Self>, OrganizationError> {
        Ok(sqlx::query_as!(
            Self,
            r#"SELECT id, name, created_at, updated_at
            FROM organizations
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?)
    }

    pub async fn get_all_organization_users(
        self,
        pool: &sqlx::PgPool,
    ) -> Result<Vec<OrganizationUser>, OrganizationError> {
        let records = sqlx::query!(
                r#"
                SELECT users.id, users.email, user_organizations.role, users.created_at, users.updated_at
                FROM users
                INNER JOIN user_organizations
                ON user_organizations.user_id = users.id
                WHERE user_organizations.organization_id = $1
                "#,
                self.id
            )
            .fetch_all(pool)
            .await?;

        let mut users = Vec::new();
        for record in records {
            let role = Role::try_from(record.role).map_err(OrganizationError::ParsingRole)?;
            users.push(OrganizationUser {
                id: record.id,
                email: record.email,
                role,
                created_at: record.created_at,
                updated_at: record.updated_at,
            });
        }

        Ok(users)
    }

    pub async fn remove_user(
        &self,
        user_id: i64,
        pool: &sqlx::PgPool,
    ) -> Result<(), OrganizationError> {
        sqlx::query!(
            "DELETE FROM user_organizations WHERE user_id = $1 AND organization_id = $2",
            user_id,
            self.id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_original_owner(
        &self,
        pool: &sqlx::PgPool,
    ) -> Result<OrganizationUser, OrganizationError> {
        let record = sqlx::query!(
            r#"
            SELECT users.id, users.email, user_organizations.role, users.created_at, users.updated_at
            FROM users
            INNER JOIN user_organizations
            ON user_organizations.user_id = users.id
            WHERE user_organizations.organization_id = $1 AND user_organizations.role = 'owner'
            Order by users.created_at
            "#,
            self.id
        )
        .fetch_one(pool)
        .await?;

        let role = Role::try_from(record.role).map_err(OrganizationError::ParsingRole)?;
        Ok(OrganizationUser {
            id: record.id,
            email: record.email,
            role,
            created_at: record.created_at,
            updated_at: record.updated_at,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationInvitation {
    pub id: i64,
    pub secret: String,
    pub organization_id: i64,
    pub email: String,
    pub role: Role,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[cfg(feature = "ssr")]
impl OrganizationInvitation {
    pub async fn create(
        organization_id: i64,
        email: &str,
        role: Role,
        pool: &sqlx::PgPool,
    ) -> Result<OrganizationInvitation, OrganizationError> {
        let record = sqlx::query!(
            r#"
                INSERT INTO organization_invitations (organization_id, email, role)
                VALUES ($1, $2, $3)
                RETURNING id, secret, organization_id, email, role, created_at, updated_at
                "#,
            organization_id,
            email,
            role.to_string()
        )
        .fetch_one(pool)
        .await?;

        let role = Role::try_from(record.role).map_err(OrganizationError::ParsingRole)?;

        Ok(OrganizationInvitation {
            id: record.id,
            secret: record.secret,
            organization_id: record.organization_id,
            email: record.email,
            role,
            created_at: record.created_at,
            updated_at: record.updated_at,
        })
    }

    pub async fn get_with_secret(
        secret: String,
        pool: &sqlx::PgPool,
    ) -> Result<Option<OrganizationInvitation>, OrganizationError> {
        let record = sqlx::query!(
            r#"
                SELECT id, secret, organization_id, email, role, created_at, updated_at
                FROM organization_invitations
                WHERE secret = $1 and deleted_at is null
                "#,
            secret
        )
        .fetch_optional(pool)
        .await?;

        if let Some(record) = record {
            let role = Role::try_from(record.role).map_err(OrganizationError::ParsingRole)?;

            Ok(Some(OrganizationInvitation {
                id: record.id,
                secret: record.secret,
                organization_id: record.organization_id,
                email: record.email,
                role,
                created_at: record.created_at,
                updated_at: record.updated_at,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn delete(self, pool: &sqlx::PgPool) -> Result<(), OrganizationError> {
        sqlx::query!(
            r#"
                UPDATE organization_invitations
                SET deleted_at = now()
                WHERE id = $1
                "#,
            self.id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_all_for_organization(
        id: i64,
        pool: &sqlx::PgPool,
    ) -> Result<Vec<OrganizationInvitation>, OrganizationError> {
        let records = sqlx::query!(
            r#"
                SELECT id, secret, organization_id, email, role, created_at, updated_at
                FROM organization_invitations
                WHERE organization_id = $1 and deleted_at is null
                "#,
            id
        )
        .fetch_all(pool)
        .await?;

        let mut result = Vec::new();
        for record in records {
            let role = Role::try_from(record.role).map_err(OrganizationError::ParsingRole)?;
            result.push(OrganizationInvitation {
                id: record.id,
                secret: record.secret,
                organization_id: record.organization_id,
                email: record.email,
                role,
                created_at: record.created_at,
                updated_at: record.updated_at,
            });
        }

        Ok(result)
    }
}
