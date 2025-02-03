use leptos::prelude::ServerFnError;
use leptos::*;
use models::user::User;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use models::{auth::Session, organization::Role};

#[cfg(feature = "ssr")]
const USER_COOKIE_NAME: &str = "context-website-user-1239999";

#[cfg(feature = "ssr")]
use rusty_paseto::core::{Key, Local, PasetoSymmetricKey, V4};

#[cfg(feature = "ssr")]
static PASETO_KEY: std::sync::LazyLock<PasetoSymmetricKey<V4, Local>> =
    std::sync::LazyLock::new(|| {
        PasetoSymmetricKey::<V4, Local>::from(Key::from(
            std::env::var("PASETO_KEY")
                .expect("missing `PASETO_KEY` env variable")
                .as_bytes(),
        ))
    });

#[cfg(feature = "ssr")]
pub async fn login_user(user: &models::user::User, pool: &sqlx::PgPool) -> anyhow::Result<()> {
    use http::{header, HeaderValue};
    use models::auth::Session;
    use prelude::expect_context;
    use rusty_paseto::{
        core::{Local, V4},
        generic::{CustomClaim, ExpirationClaim},
        prelude::PasetoBuilder,
    };
    use time::format_description::well_known::Rfc3339;

    let response = expect_context::<leptos_axum::ResponseOptions>();

    let session = Session::create(Some(user.id), None, pool).await?;

    let key = &*PASETO_KEY;
    // TODO: @silas set this time to seconds and verify the parsing fails
    let expiration = time::OffsetDateTime::now_utc() + time::Duration::days(5); // This 5 day expiration lines up with the 5 day session expiration set in the
                                                                                // database when inserting a new session
    let token = PasetoBuilder::<V4, Local>::default()
        .set_claim(ExpirationClaim::try_from(expiration.format(&Rfc3339)?)?)
        .set_claim(CustomClaim::try_from(("session_id", session.id)).unwrap())
        .build(key)?;

    let cookie = if *crate::PUBLIC_DEV_MODE {
        cookie::Cookie::build(((USER_COOKIE_NAME).to_string(), token))
            .domain("127.0.0.1")
            .path("/")
            .secure(false)
            .http_only(true)
    } else {
        cookie::Cookie::build(((USER_COOKIE_NAME).to_string(), token))
            .domain("getsundry.app")
            .path("/")
            .secure(true)
            .http_only(true)
    };

    response.insert_header(
        header::SET_COOKIE,
        HeaderValue::from_str(&cookie.to_string()).unwrap(),
    );

    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn logout_user(pool: &sqlx::PgPool) -> anyhow::Result<()> {
    use anyhow::Context;
    use http::{header, HeaderValue};
    use models::auth::Session;
    use prelude::expect_context;
    use rusty_paseto::{
        core::{Local, V4},
        generic::{CustomClaim, ExpirationClaim},
        prelude::PasetoBuilder,
    };
    use time::format_description::well_known::Rfc3339;

    let response = expect_context::<leptos_axum::ResponseOptions>();

    let session_id = get_session_id()?.context("no session found")?;
    if let Some(session) = Session::get_with_id(session_id, pool).await? {
        session.logout(pool).await?;
    }

    let key = &*PASETO_KEY;
    // TODO: @silas set this time to seconds and verify the parsing fails
    let expiration = time::OffsetDateTime::now_utc(); // expire now to remove.
    let token = PasetoBuilder::<V4, Local>::default()
        .set_claim(ExpirationClaim::try_from(expiration.format(&Rfc3339)?)?)
        .set_claim(CustomClaim::try_from(("session_id", "")).unwrap())
        .build(key)?;

    let cookie = if *crate::PUBLIC_DEV_MODE {
        cookie::Cookie::build(((USER_COOKIE_NAME).to_string(), token))
            .domain("127.0.0.1")
            .path("/")
            .secure(false)
            .http_only(true)
    } else {
        cookie::Cookie::build(((USER_COOKIE_NAME).to_string(), token))
            .domain("getsundry.app")
            .path("/")
            .secure(true)
            .http_only(true)
    };

    response.insert_header(
        header::SET_COOKIE,
        HeaderValue::from_str(&cookie.to_string()).unwrap(),
    );

    Ok(())
}

#[cfg(feature = "ssr")]
pub fn get_token_from_headers(headers: &axum::http::HeaderMap) -> Option<String> {
    headers
        .get(http::header::COOKIE)
        .and_then(|cookie_header| cookie_header.to_str().ok())
        .and_then(|cookie_str| {
            cookie::Cookie::split_parse(cookie_str)
                .filter_map(Result::ok)
                .find(|cookie| cookie.name() == USER_COOKIE_NAME)
                .map(|cookie| cookie.value().to_string())
        })
}

#[cfg(feature = "ssr")]
pub fn get_session_id_from_headers(headers: &axum::http::HeaderMap) -> anyhow::Result<Option<i64>> {
    use rusty_paseto::prelude::*;
    if let Some(token) = get_token_from_headers(headers) {
        let key = &*PASETO_KEY;
        let value = PasetoParser::<V4, Local>::default().parse(&token, key);

        if let Ok(value) = value {
            match value["session_id"].as_i64() {
                Some(v) => Ok(Some(v as i64)),
                None => anyhow::bail!("malformed paseto"),
            }
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

#[cfg(feature = "ssr")]
pub fn get_session_id() -> anyhow::Result<Option<i64>> {
    use prelude::use_context;

    if let Some(req) = use_context::<axum::http::request::Parts>() {
        get_session_id_from_headers(&req.headers)
    } else {
        Ok(None)
    }
}

#[cfg(feature = "ssr")]
pub async fn get_logged_in_user_organization_role(
    organization_id: i64,
) -> anyhow::Result<Option<Role>> {
    use prelude::expect_context;

    use crate::AppState;
    if let Some(session_id) = get_session_id()? {
        let state: AppState = expect_context();
        Ok(
            Role::get_with_session_and_organization(session_id, organization_id, &state.pool)
                .await?,
        )
    } else {
        Ok(None)
    }
}

#[server(APIGetLoggedInuser)]
pub async fn get_logged_in_user() -> Result<Option<User>, ServerFnError> {
    use prelude::use_context;

    use crate::{error::LogAndMapInternalServerError, AppState};

    if let Some(session_id) = get_session_id().ise()? {
        let state: AppState = use_context().ok_or(ServerFnError::new("missing app context"))?;
        get_logged_in_user_with_session_id_and_pool(session_id, &state.pool)
            .await
            .ise()
    } else {
        Ok(None)
    }
}

#[cfg(feature = "ssr")]
pub async fn get_logged_in_user_with_session_id_and_pool(
    session_id: i64,
    pool: &sqlx::PgPool,
) -> anyhow::Result<Option<User>> {
    let session = if let Some(session) = Session::get_with_id(session_id, &pool).await? {
        session
    } else {
        return Ok(None);
    };

    if !session.logged_in || session.is_expired(pool).await? {
        return Ok(None);
    }

    let user = User::get_user_by_session_id(session_id, &pool).await?;
    Ok(user)
}

#[cfg(feature = "ssr")]
pub async fn create_anonymous_session_with_data(
    pool: &sqlx::PgPool,
    data: serde_json::Value,
) -> anyhow::Result<i64> {
    use http::{header, HeaderValue};
    use models::auth::Session;
    use prelude::expect_context;
    use rusty_paseto::{
        core::{Local, V4},
        generic::{CustomClaim, ExpirationClaim},
        prelude::PasetoBuilder,
    };

    let response = expect_context::<leptos_axum::ResponseOptions>();

    let key = &*PASETO_KEY;
    let expiration = time::OffsetDateTime::now_utc() + time::Duration::hours(1); // This 5 day expiration lines up with the 5 day session expiration set in the

    let session = Session::create(None, Some(data), pool).await?;

    // database when inserting a new session
    let token = PasetoBuilder::<V4, Local>::default()
        .set_claim(ExpirationClaim::try_from(
            expiration.format(&time::format_description::well_known::Rfc3339)?,
        )?)
        .set_claim(CustomClaim::try_from(("session_id", session.id)).unwrap())
        .build(key)?;

    let cookie = if *crate::PUBLIC_DEV_MODE {
        cookie::Cookie::build(((USER_COOKIE_NAME).to_string(), token))
            .domain("127.0.0.1")
            .path("/")
            .secure(false)
            .http_only(true)
    } else {
        cookie::Cookie::build(((USER_COOKIE_NAME).to_string(), token))
            .domain("getsundry.app")
            .path("/")
            .secure(true)
            .http_only(true)
    };

    response.insert_header(
        header::SET_COOKIE,
        HeaderValue::from_str(&cookie.to_string()).unwrap(),
    );

    Ok(session.id)
}
