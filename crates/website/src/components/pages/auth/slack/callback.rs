use leptos::prelude::*;
use leptos_router::hooks::use_query;
use serde::{Deserialize, Serialize};

use crate::{
    components::atoms::loading::LoadingWheelCover, utils::get_maybe_server_fn_user_error,
    PUBLIC_DEV_MODE, PUBLIC_SLACK_CLIENT_ID,
};
use leptos_router::params::Params;

#[derive(Clone, Params, PartialEq, Deserialize, Serialize, Debug)]
pub struct SlackAuthCallbackQueryParams {
    state: Option<i64>,
    code: Option<String>,
}

// There are other fields on this but we don't care about them
#[cfg(feature = "ssr")]
#[derive(serde::Deserialize, Debug)]
pub struct AuthedUser {
    id: String,
    access_token: String,
}

/// The information concerning our installed organization slack app
// There are other fields on this but we don't care about them
#[cfg(feature = "ssr")]
#[derive(serde::Deserialize, Debug)]
struct SlackAppTokenResponse {
    access_token: Option<String>,
    authed_user: AuthedUser,
}

/// Information concerning the installing/Oauth user
// There are other fields on this but we don't care about them
#[cfg(feature = "ssr")]
#[derive(serde::Deserialize, Debug)]
struct SlackUserTokenResponse {
    email: String,
    sub: String,
}

#[server(AuthCallback)]
pub async fn auth_callback(
    query: SlackAuthCallbackQueryParams,
) -> Result<Option<String>, ServerFnError> {
    use anyhow::Context;
    use models::slack::app_installation::AppInstallation;

    use crate::{error::LogAndMapInternalServerError, AppState};

    let state: AppState = use_context().context("getting context").ise()?;
    let organization_id = query.state.context("no state in Slack callback").ise()?;

    // If user is None they are signing up
    let user = crate::auth::utils::get_logged_in_user()
        .await?
        .context("slack callback needs a user")
        .ise()?;

    let client = reqwest::Client::new();

    let params = [
        ("client_id", PUBLIC_SLACK_CLIENT_ID.to_string()),
        (
            "client_secret",
            std::env::var("SLACK_CLIENT_SECRET")
                .context("missing `SLACK_APP_CLIENT_SECRET` env variable")
                .ise_with_user_message("bad redirect from Slack")?,
        ),
        (
            "code",
            query
                .code
                .context("no Slack code in url params")
                .ise_with_user_message("bad redirect from Slack")?,
        ),
    ];

    let redirect_url = if *PUBLIC_DEV_MODE {
        "https://127.0.0.1:3002/slack/auth-callback".to_string()
    } else {
        "https://getsundry.app/slack/auth-callback".to_string()
    };

    let app_res: SlackAppTokenResponse = client
        .post(format!(
            "https://slack.com/api/oauth.v2.access?redirect_uri={redirect_url}"
        ))
        .header("Accept", "application/json")
        .form(&params)
        .send()
        .await
        .context("getting Slack access token 1")
        .ise()?
        .json()
        .await
        .context("getting Slack access token 2")
        .ise()?;

    let user_res: SlackUserTokenResponse = client
        .get("https://slack.com/api/openid.connect.userInfo")
        .header("Accept", "application/json")
        .header(
            "Authorization",
            format!("Bearer {}", app_res.authed_user.access_token),
        )
        .send()
        .await
        .context("getting Slack user identity 1")
        .ise()?
        .json()
        .await
        .context("getting Slack user identity 2")
        .ise()?;

    let mut conn = (state.pool.acquire().await).ise()?;
    user.add_slack_oauth(
        &app_res.authed_user.access_token,
        &user_res.sub,
        &user_res.email,
        &mut conn,
    )
    .await
    .context("adding Slack oauth to existing user")
    .ise()?;

    // If we are installing the app we will get an app access token.
    if let Some(app_access_token) = app_res.access_token {
        AppInstallation::create(&app_access_token, organization_id, &state.pool)
            .await
            .ise()?;
    }

    Ok(Some("/dashboard".to_string()))
}

#[component]
pub fn AuthCallback(refetch_user: Box<dyn Fn(Option<String>)>) -> impl IntoView {
    let action = Action::new(move |_| async move {
        let query = use_query::<SlackAuthCallbackQueryParams>();
        match query.get_untracked() {
            Ok(query) => auth_callback(query).await,
            _ => Err(ServerFnError::new("No query params")),
        }
    });

    let value = action.value();

    // Redirect on successful login
    Effect::new(move |_| {
        action.dispatch(());
    });

    Effect::new(move |_| {
        if let Some(Ok(path)) = value.get() {
            refetch_user(path);
        }
    });

    view! {
        {move || {
            if let Some(msg) = get_maybe_server_fn_user_error(value.get()) {
                view! {
                    <h3>"Something has gone wrong"</h3>
                    <div>{msg}</div>
                }
                    .into_any()
            } else {
                view! { <LoadingWheelCover /> }.into_any()
            }
        }}
    }
}
