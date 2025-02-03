use leptos::prelude::*;
use models::{
    github::app_installation::AppInstallation as GithubAppInstallation, organization::Organization,
    slack, slack::app_installation::AppInstallation as SlackAppInstallation,
};
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::auth::utils::get_logged_in_user_organization_role;
use crate::components::atoms::tag::ButtonAction;
use crate::components::molecules::source_card::{Action as SourceAction, SourceCard, SourceType};

use crate::{
    PUBLIC_DEV_MODE, PUBLIC_GITHUB_APP_CLIENT_ID, PUBLIC_GITHUB_APP_NAME, PUBLIC_SLACK_CLIENT_ID,
    PUBLIC_SLACK_SCOPES, PUBLIC_SLACK_USER_SCOPES,
};

#[server(GetGithubIntegrations, "/api")]
async fn get_github_app_installation(
    organization_id: i64,
) -> Result<Vec<GithubAppInstallation>, ServerFnError> {
    use crate::{
        auth::utils::get_logged_in_user_organization_role, error::LogAndMapInternalServerError,
    };

    let role = get_logged_in_user_organization_role(organization_id)
        .await
        .ise()?;
    if role.is_none() {
        return Err(ServerFnError::new("not authorized"));
    }

    let state: crate::AppState = expect_context();
    GithubAppInstallation::get_with_organization_id(organization_id, &state.pool)
        .await
        .ise()
}

#[server(GetSlackIntegrations, "/api")]
async fn get_slack_app_installation(
    organization_id: i64,
) -> Result<Vec<SlackAppInstallation>, ServerFnError> {
    use crate::{
        auth::utils::get_logged_in_user_organization_role, error::LogAndMapInternalServerError,
    };

    let role = get_logged_in_user_organization_role(organization_id)
        .await
        .ise()?;
    if role.is_none() {
        return Err(ServerFnError::new("not authorized"));
    }

    let state: crate::AppState = expect_context();
    SlackAppInstallation::get_with_organization_id(organization_id, &state.pool)
        .await
        .ise()
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AppInstallations {
    pub github: Vec<GithubAppInstallation>,
    pub slack: Vec<SlackAppInstallation>,
    // pub google_docs: Vec<AppInstallation>,
}

#[server(RemoveIntegration, "/api")]
async fn remove_github_app_installation(installation_id: i64) -> Result<(), ServerFnError> {
    use crate::error::LogAndMapInternalServerError;
    use leptos_axum;
    let state: crate::AppState = expect_context();

    let installation =
        GithubAppInstallation::get_with_installation_id(installation_id, &state.pool)
            .await
            .ise()?;

    // Double check the user has acceess to the organization
    let role = get_logged_in_user_organization_role(installation.organization_id)
        .await
        .ise()?;
    if role.is_none() {
        return Err(ServerFnError::new("not authorized"));
    }

    // This sends user to their github settings page to manually delete the
    // installation We will need to take the action for the user and remove from DB
    // with callback.
    match installation.delete(&state.pool).await.ise() {
        Ok(_) => {
            leptos_axum::redirect(&format!(
                "https://github.com/settings/installations/{}",
                installation_id
            ));
            Ok(())
        }
        Err(_) => Err(ServerFnError::new("Failed to remove installation")),
    }
}

#[component]
fn GitHubAppInstallationLink(installation_id: Option<i64>) -> impl IntoView {
    let organization: Organization = expect_context();

    let remove = Action::new(move |_: &()| async move {
        remove_github_app_installation(installation_id.expect("No installation id provided")).await
    });

    view! {
        {match installation_id {
            Some(_) => {
                view! {
                    <SourceCard
                        icon=view! { <img src="/images/github.svg" /> }
                        title=view! { "GitHub" }
                        description=view! {
                            "Connect your Github repository, issues, pull requests and comments across all three."
                        }
                        action=SourceAction {
                            variant: SourceType::Disconnect,
                            btn_action: ButtonAction::Action(
                                Box::new({
                                    move || {
                                        remove.dispatch(());
                                    }
                                }),
                            ),
                        }
                    />
                }
                    .into_any()
            }
            None => {
                view! {
                    <SourceCard
                        icon=view! { <img src="/images/github.svg" /> }
                        title=view! { "GitHub" }
                        description=view! {
                            "Connect your Github repository, issues, pull requests and comments across all three."
                        }
                        action=SourceAction {
                            variant: SourceType::Connect,
                            btn_action: ButtonAction::Link(
                                format!(
                                    "https://github.com/apps/{}/installations/new?state={}",
                                    *PUBLIC_GITHUB_APP_NAME,
                                    organization.id,
                                ),
                            ),
                        }
                    />
                }
                    .into_any()
            }
        }}
    }
}

#[server(GithubUserConnected, "/api")]
async fn get_github_user_connected() -> Result<Option<String>, ServerFnError> {
    use crate::auth::utils::get_logged_in_user;
    use crate::{error::LogAndMapInternalServerError, AppState};
    use models::user::github_username_for_user_id;
    let state: AppState = expect_context();

    let pool = &state.pool;

    let github_user_name = match get_logged_in_user().await.ise()? {
        Some(user) => github_username_for_user_id(user.id, pool)
            .await
            .ok()
            .flatten(),
        None => return Ok(None),
    };

    Ok(github_user_name)
}

#[server(SlackUserConnected, "/api")]
async fn get_slack_user_connected() -> Result<Option<String>, ServerFnError> {
    use crate::auth::utils::get_logged_in_user;
    use crate::{error::LogAndMapInternalServerError, AppState};
    use models::user::slack_user_id_for_user_id;
    let state: AppState = expect_context();

    let pool = &state.pool;

    let slack_user_id = match get_logged_in_user().await.ise()? {
        Some(user) => slack_user_id_for_user_id(user.id, pool)
            .await
            .ok()
            .flatten(),
        None => return Ok(None),
    };

    Ok(slack_user_id)
}

#[component]
pub fn GithubUserConnectLink(installation_id: Option<i64>) -> impl IntoView {
    let user_connected = Resource::new(
        || (),
        move |_| async move { get_github_user_connected().await },
    );

    view! {
        {match installation_id {
            Some(_) => {
                view! {
                    <Suspense fallback=move || {
                        view! { "Loading Connection..." }
                    }>
                        {move || {
                            let description = match user_connected.get() {
                                Some(Ok(Some(username))) => {
                                    view! { {format!("Connected as {}", username)} }.into_any()
                                }
                                _ => {
                                    view! {
                                        "Connect your Github repository, issues, pull requests and comments
                                        across all three."
                                    }
                                        .into_any()
                                }
                            };
                            let action = match user_connected.get() {
                                Some(Ok(Some(_))) => {
                                    SourceAction {
                                        variant: SourceType::Disconnect,
                                        btn_action: ButtonAction::Link(String::from("todo")),
                                    }
                                }
                                _ => {
                                    SourceAction {
                                        variant: SourceType::Connect,
                                        btn_action: ButtonAction::Link(
                                            format!(
                                                "https://github.com/login/oauth/authorize?client_id={}",
                                                &*PUBLIC_GITHUB_APP_CLIENT_ID,
                                            ),
                                        ),
                                    }
                                }
                            };
                            view! {
                                <SourceCard
                                    icon=view! { <img src="/images/github.svg" /> }
                                    title=view! { "GitHub" }
                                    description=description
                                    action=action
                                />
                            }
                                .into_any()
                        }}
                    </Suspense>
                }
                    .into_any()
            }
            None => view! {}.into_any(),
        }}
    }
}

#[server(RemoveSlackIntegration, "/api")]
async fn remove_slack_app_installation(organization_id: i64) -> Result<(), ServerFnError> {
    use crate::error::LogAndMapInternalServerError;
    let state: crate::AppState = expect_context();

    // Double check the user has acceess to the organization
    let role = get_logged_in_user_organization_role(organization_id)
        .await
        .ise()?;
    if role.is_none() {
        return Err(ServerFnError::new("not authorized"));
    }

    let installations = slack::app_installation::AppInstallation::get_with_organization_id(
        organization_id,
        &state.pool,
    )
    .await
    .ise()?;

    for installation in installations {
        installation.delete(&state.pool).await.ise()?;
    }

    Ok(())
}

#[component]
fn SlackAppInstallationLink(
    source_type: SourceType,
    refetch_signal: RwSignal<bool>,
) -> impl IntoView {
    let organization: Organization = expect_context();

    let remove = Action::new(move |_: &()| async move {
        let _ = remove_slack_app_installation(organization.id).await;
        refetch_signal.set(!refetch_signal.get_untracked());
    });

    let redirect_url = if *PUBLIC_DEV_MODE {
        "https://127.0.0.1:3002/slack/auth-callback".to_string()
    } else {
        "https://getsundry.app/slack/auth-callback".to_string()
    };

    view! {
        {match source_type {
            SourceType::Connect => {
                view! {
                    <SourceCard
                        icon=view! { <img src="/images/slack.svg" /> }
                        title=view! { "Slack" }
                        description=view! { "Connect your Slack workspace, channels and groups." }
                        action=SourceAction {
                            variant: SourceType::Connect,
                            btn_action: ButtonAction::Link(
                                format!(
                                    "https://slack.com/oauth/v2/authorize?client_id={}&scope={}&user_scope={}&state={}&redirect_uri={redirect_url}",
                                    *PUBLIC_SLACK_CLIENT_ID,
                                    *PUBLIC_SLACK_SCOPES,
                                    *PUBLIC_SLACK_USER_SCOPES,
                                    organization.id,
                                ),
                            ),
                        }
                    />
                }
                    .into_any()
            }
            _ => {
                view! {
                    <SourceCard
                        icon=view! { <img src="/images/slack.svg" /> }
                        title=view! { "Slack" }
                        description=view! { "Connect your Slack workspace, channels and groups." }
                        action=SourceAction {
                            variant: SourceType::Disconnect,
                            btn_action: ButtonAction::Action(
                                Box::new({
                                    move || {
                                        remove.dispatch(());
                                    }
                                }),
                            ),
                        }
                    />
                }
                    .into_any()
            }
        }}
    }
}

#[component]
fn SlackUserConnectLink(installation_id: Option<i64>) -> impl IntoView {
    let organization = expect_context::<Organization>();

    let user_connected = Resource::new(
        || (),
        move |_| async move { get_slack_user_connected().await },
    );

    let redirect_url = if *PUBLIC_DEV_MODE {
        "https://127.0.0.1:3002/slack/auth-callback".to_string()
    } else {
        "https://getsundry.app/slack/auth-callback".to_string()
    };

    view! {
        {match installation_id {
            Some(_) => {
                view! {
                    <Suspense fallback=move || {
                        view! { "Loading " }
                    }>
                        {move || {
                            let description = match user_connected.get() {
                                Some(Ok(Some(username))) => {
                                    view! { {format!("Connected as {}", username)} }.into_any()
                                }
                                _ => {
                                    view! { "Connect your Slack workspace, channels and groups." }
                                        .into_any()
                                }
                            };
                            let action = match user_connected.get() {
                                Some(Ok(Some(_))) => {
                                    SourceAction {
                                        variant: SourceType::Disconnect,
                                        btn_action: ButtonAction::Link(String::from("todo")),
                                    }
                                }
                                _ => {
                                    SourceAction {
                                        variant: SourceType::Connect,
                                        btn_action: ButtonAction::Link(
                                            format!(
                                                "https://slack.com/oauth/v2/authorize?client_id={}&user_scope={}&state={}&redirect_uri={redirect_url}",
                                                *PUBLIC_SLACK_CLIENT_ID,
                                                *PUBLIC_SLACK_USER_SCOPES,
                                                organization.id,
                                            ),
                                        ),
                                    }
                                }
                            };
                            view! {
                                <SourceCard
                                    icon=view! { <img src="/images/slack.svg" /> }
                                    title=view! { "Slack" }
                                    description=description
                                    action=action
                                />
                            }
                                .into_any()
                        }}
                    </Suspense>
                }
                    .into_any()
            }
            _ => view! {}.into_any(),
        }}
    }
}

#[component]
pub fn OrgSources() -> impl IntoView {
    let organization: Organization = expect_context();

    let refetch_signal = RwSignal::new(false);

    let organization_id = organization.id;
    let integrations = Resource::new(
        move || refetch_signal.get(),
        move |_| async move {
            let github = get_github_app_installation(organization_id).await;
            let slack = get_slack_app_installation(organization_id).await;
            AppInstallations {
                github: github.unwrap_or_default(),
                slack: slack.unwrap_or_default(),
                // google_docs: vec![],
            }
        },
    );

    view! {
        <Suspense fallback=move || {
            view! { "Loading integrations..." }
        }>
            {move || {
                let installations = integrations.get();
                let github_installation_id = installations
                    .clone()
                    .and_then(|i| i.github.last().and_then(|x| Some(x.installation_id.clone())));
                let github_action = view! {
                    <GitHubAppInstallationLink installation_id=github_installation_id />
                }
                    .into_any();
                let slack_action = if let Some(installations) = installations.clone() {
                    match installations.slack.last() {
                        Some(_) => {
                            view! {
                                <SlackAppInstallationLink
                                    source_type=SourceType::Disconnect
                                    refetch_signal=refetch_signal
                                />
                            }
                                .into_any()
                        }
                        None => {
                            view! {
                                <SlackAppInstallationLink
                                    source_type=SourceType::Connect
                                    refetch_signal=refetch_signal
                                />
                            }
                                .into_any()
                        }
                    }
                } else {
                    view! { <div></div> }.into_any()
                };
                view! {
                    <div class="flex flex-col gap-5 divide-y divide-neutral-600">
                        <div class="py-4">{github_action}</div>
                        <div class="py-4">{slack_action}</div>
                    </div>
                }
            }}
        </Suspense>
    }
}

#[component]
pub fn UserSources() -> impl IntoView {
    let organization: Organization = expect_context();

    let organization_id = organization.id;
    let integrations = Resource::new(
        || (),
        move |_| async move {
            let github = get_github_app_installation(organization_id).await;
            let slack = get_slack_app_installation(organization_id).await;
            AppInstallations {
                github: github.unwrap_or_default(),
                slack: slack.unwrap_or_default(),
                // google_docs: vec![],
            }
        },
    );

    view! {
        <Suspense fallback=move || {
            view! { "Loading integrations..." }
        }>
            {move || {
                let installations = integrations.get();
                let github_installation_id = installations
                    .clone()
                    .and_then(|i| i.github.last().and_then(|x| Some(x.id.clone())));
                let github_action = view! {
                    <GithubUserConnectLink installation_id=github_installation_id />
                };
                let slack_installation_id = installations
                    .clone()
                    .and_then(|i| i.slack.last().and_then(|x| Some(x.id.clone())));
                let slack_action = view! {
                    <SlackUserConnectLink installation_id=slack_installation_id />
                };
                view! {
                    <div class="flex flex-col gap-5 divide-y divide-neutral-600">
                        <div class="py-4">{github_action}</div>
                        <div class="py-4">{slack_action}</div>
                    </div>
                }
            }}
        </Suspense>
    }
}

#[component]
pub fn ComingSoon() -> impl IntoView {
    view! {
        <div class="flex flex-col">
            <div class="flex flex-row gap-5 justify-center flex-wrap">
                <SourceCard
                    icon=view! { <img src="/images/discord.svg" /> }
                    title=view! { "Discord" }
                    description=view! { "" }
                />
                <SourceCard
                    icon=view! { <img src="/images/confluence.svg" /> }
                    title=view! { "Confluence" }
                    description=view! { "" }
                />
                <SourceCard
                    icon=view! { <img src="/images/google_drive.svg" /> }
                    title=view! { "Google Drive" }
                    description=view! { "" }
                />
                <SourceCard
                    icon=view! { <img src="/images/gmail.svg" /> }
                    title=view! { "Gmail" }
                    description=view! { "" }
                />
                <SourceCard
                    icon=view! { <img src="/images/guru.svg" /> }
                    title=view! { "Guru" }
                    description=view! { "" }
                />
                <SourceCard
                    icon=view! { <img src="/images/jira.svg" /> }
                    title=view! { "Jira" }
                    description=view! { "" }
                />
                <SourceCard
                    icon=view! { <img src="/images/salesforce.svg" /> }
                    title=view! { "Salesforce" }
                    description=view! { "" }
                />
                <SourceCard
                    icon=view! { <img src="/images/sharepoint.svg" /> }
                    title=view! { "Sharepoint" }
                    description=view! { "" }
                />
                <SourceCard
                    icon=view! { <img src="/images/slab.svg" /> }
                    title=view! { "Slab" }
                    description=view! { "" }
                />
                <SourceCard
                    icon=view! { <img src="/images/stack_overflow.svg" /> }
                    title=view! { "Stack Overflow" }
                    description=view! { "" }
                />
                <SourceCard
                    icon=view! { <img src="/images/zendesk.svg" /> }
                    title=view! { "Zendesk" }
                    description=view! { "" }
                />
                <SourceCard
                    icon=view! { <img src="/images/teams.svg" /> }
                    title=view! { "Teams" }
                    description=view! { "" }
                />
                <SourceCard
                    icon=view! { <img src="/images/youtube.svg" /> }
                    title=view! { "Youtube" }
                    description=view! { "" }
                />
                <SourceCard
                    icon=view! { <img src="/images/notion.svg" /> }
                    title=view! { "Notion" }
                    description=view! { "" }
                />
                <SourceCard
                    icon=view! { <img src="/images/question_answer.svg" /> }
                    title=view! { "Web Crawler" }
                    description=view! { "" }
                />
            </div>
        </div>
    }
}
