use leptos::prelude::*;
use leptos_router::hooks::use_query;
use serde::{Deserialize, Serialize};

use crate::{components::atoms::loading::LoadingWheelCover, utils::get_maybe_server_fn_user_error};
use leptos_router::params::Params;

#[cfg(feature = "ssr")]
use crate::PUBLIC_GITHUB_APP_NAME;

#[cfg(feature = "ssr")]
use crate::PUBLIC_GITHUB_APP_CLIENT_ID;

#[derive(Clone, Params, PartialEq, Deserialize, Serialize, Debug)]
pub struct GitHubAuthCallbackQueryParams {
    code: Option<String>,
    installation_id: Option<i64>,
    setup_action: Option<String>,
    state: Option<i64>,
}

// There are other fields on this but we don't care about them
#[cfg(feature = "ssr")]
#[derive(serde::Deserialize, Debug)]
struct GitHubAccessTokenResponse {
    access_token: String,
}

// There are other fields on this but we don't care about them
#[cfg(feature = "ssr")]
#[derive(serde::Deserialize, Debug)]
struct GitHubEmail {
    email: String,
    primary: bool,
}

#[cfg(feature = "ssr")]
async fn get_primary_email_address(access_token: &str) -> Result<Option<String>, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut page = 1;
    loop {
        let user_emails: Vec<GitHubEmail> = client
            .get(format!(
                "https://api.github.com/user/emails?per_page=100&page={page}"
            ))
            .header("X-GitHub-Api-Version", "2022-11-28")
            .header("Accept", "application/vnd.github+json")
            .header("User-Agent", *PUBLIC_GITHUB_APP_NAME)
            .bearer_auth(access_token)
            .send()
            .await?
            .json()
            .await?;

        let maybe_primary_email = user_emails.iter().find(|email| email.primary);

        if let Some(primary_email) = maybe_primary_email {
            return Ok(Some(primary_email.email.clone()));
        }

        if user_emails.len() < 100 {
            break;
        }
        page += 1;
    }

    Ok(None)
}

#[server(AuthCallback)]
pub async fn auth_callback(
    query: GitHubAuthCallbackQueryParams,
) -> Result<Option<String>, ServerFnError> {
    use crate::components::pages::auth::invite::add_user_from_invitation;
    use anyhow::Context;
    use models::github::app_installation::AppInstallation;

    use crate::{auth::utils::login_user, error::LogAndMapInternalServerError, AppState};

    let state: AppState = use_context().context("getting context").ise()?;
    let session_id = crate::auth::utils::get_session_id().ise()?;

    let session = if let Some(session_id) = session_id {
        models::auth::Session::get_with_id(session_id, &state.pool)
            .await
            .ise()?
            .context("getting session")
            .ise()
            .ok()
    } else {
        None
    };

    let invite_secret = session.and_then(|session| session.data.get("org_invite_secret").cloned());

    let invite_secret: Option<String> = invite_secret
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .and_then(|v| match v.as_str() {
            "" => None,
            _ => Some(v),
        });

    // If user is None they are signing up
    let user = crate::auth::utils::get_logged_in_user().await?;
    let is_user_logged_in = user.is_some();

    let client = reqwest::Client::new();

    let params = [
        ("client_id", (&*PUBLIC_GITHUB_APP_CLIENT_ID).to_string()),
        (
            "client_secret",
            std::env::var("GITHUB_APP_CLIENT_SECRET")
                .expect("missing `GITHUB_APP_CLIENT_SECRET` env variable"),
        ),
        (
            "code",
            query
                .code
                .context("no GitHub code in url params")
                .ise_with_user_message("bad redirect from GitHub")?,
        ),
    ];

    let res: GitHubAccessTokenResponse = client
        .post("https://github.com/login/oauth/access_token")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("Accept", "application/json")
        .form(&params)
        .send()
        .await
        .context("getting GitHub access token")
        .ise()?
        .json()
        .await
        .context("getting GitHub access token")
        .ise()?;

    let github_user: github_types::User = client
        .get("https://api.github.com/user")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", *PUBLIC_GITHUB_APP_NAME)
        .bearer_auth(&res.access_token)
        .send()
        .await
        .context("getting GitHub user")
        .ise()?
        .json()
        .await
        .context("getting GitHub user")
        .ise()?;

    let github_email_address = get_primary_email_address(&res.access_token)
        .await
        .context("getting GitHub email address")
        .ise()?
        .context("empty GitHub email address")
        .ise()?;

    let mut conn = (state.pool.acquire().await).ise()?;
    // If the user exists add github as an oauth method
    // If the user does not exist create the user
    let user = if let Some(user) = user {
        user.add_github_oauth(
            github_user.id,
            &github_user.login,
            &res.access_token,
            &mut conn,
        )
        .await
        .context("adding GitHub oauth to existing user")
        .ise()?;
        user
    } else {
        models::user::User::create_with_github(
            github_user.id,
            &github_user.login,
            github_user.name.as_deref(),
            &res.access_token,
            &github_email_address,
            &state.pool,
        )
        .await
        .context("creating new user with GitHub oauth")
        .ise()?
    };

    // If invited add them to inviting organization
    if let Some(invite_secret) = invite_secret {
        add_user_from_invitation(user.clone(), invite_secret)
            .await
            .ise()?;
    }

    // If they are redirecting here after installing our github app these parameters
    // are set
    let reroute_path = match (query.installation_id, query.setup_action, query.state) {
        (Some(installation_id), Some(setup_action), Some(organization_id)) => {
            // They should only ever get here after an installation
            // If they get here for any other reason we didn't ask them to and we error
            if setup_action.as_str() != "install" {
                return Err(ServerFnError::new("setup_action was not install"));
            }
            AppInstallation::create(installation_id, organization_id, &state.pool)
                .await
                .context("creating new GitHub AppInstallation")
                .ise()?;
            Some(format!("/dashboard/{}/overview", organization_id))
        }
        (_, _, Some(organization_id)) => Some(format!("/dashboard/{}/overview", organization_id)),
        _ => None,
    };

    if !is_user_logged_in {
        login_user(&user, &state.pool).await.ise()?;
        Ok(Some("/dashboard".to_string()))
    } else {
        Ok(reroute_path)
    }
}

#[component]
pub fn AuthCallback(refetch_user: Box<dyn Fn(Option<String>)>) -> impl IntoView {
    let action = Action::new(move |_| async move {
        let query = use_query::<GitHubAuthCallbackQueryParams>();
        match query.get_untracked() {
            Ok(query) => auth_callback(query.clone()).await,
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
