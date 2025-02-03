use crate::components::atoms::copy::Copy;
use crate::components::atoms::tag::{ActionTag, ActionTagVariant, ButtonAction};
use crate::components::molecules::api_card::{ApiCard, ApiCardVariant};
use crate::utils::get_server_fn_user_error;
use leptos::prelude::*;
use models::{
    api::user_key::{UserKey, UserKeyWithKey},
    organization::Organization,
};

#[cfg(feature = "ssr")]
use crate::auth::utils::get_logged_in_user;
use crate::components::atoms::button::{Button, ButtonVariant};
use crate::components::molecules::modal::Modal;

#[server(GetUserAPIKeys)]
async fn get_user_api_keys(organization_id: i64) -> Result<Vec<UserKey>, ServerFnError> {
    use crate::{
        auth::utils::get_logged_in_user_organization_role, error::LogAndMapInternalServerError,
    };

    // Double check the user is part of the organization they are requesting keys
    // for It's impossible for a user to create keys for an organization they
    // don't exist in but they may have been removed from it
    let role = get_logged_in_user_organization_role(organization_id)
        .await
        .ise()?;
    if role.is_none() {
        return Err(ServerFnError::new("not authorized"));
    }

    let user = get_logged_in_user()
        .await?
        .ok_or(ServerFnError::new("not authorized"))?;

    let state: crate::AppState = expect_context();
    UserKey::get_with_user_id_organization_id(user.id, organization_id, &state.pool)
        .await
        .ise()
}

#[server(GenerateUserAPIKeyFn)]
async fn generate_user_api_key(
    organization_id: i64,
    description: String,
) -> Result<UserKeyWithKey, ServerFnError> {
    use crate::{
        auth::utils::get_logged_in_user_organization_role, error::LogAndMapInternalServerError,
    };

    // Double check the user is part of the organization they are requesting to
    // generate a key for
    let role = get_logged_in_user_organization_role(organization_id)
        .await
        .ise()?;
    if role.is_none() {
        return Err(ServerFnError::new("not authorized"));
    }

    let user = get_logged_in_user()
        .await?
        .ok_or(ServerFnError::new("not authorized"))?;

    let state: crate::AppState = expect_context();
    let mut conn = state.pool.acquire().await.ise()?;
    UserKeyWithKey::create_with_user_id_organization_id(
        user.id,
        organization_id,
        &description,
        false,
        &mut conn,
    )
    .await
    .ise()
}

#[server(RemoveUserAPIKeyFn)]
async fn remove_user_api_key(key_id: String) -> Result<(), ServerFnError> {
    use crate::error::LogAndMapInternalServerError;
    let state: crate::AppState = expect_context();

    let key = UserKey::get_with_key_id(&key_id, &state.pool).await.ise()?;
    match key {
        Some(key) => key.delete(&state.pool).await.ise(),
        None => Err(ServerFnError::new("key not found")),
    }
}

#[component]
fn GenerateUserAPIKey(
    set_show_modal: WriteSignal<bool>,
    refresh: (ReadSignal<bool>, WriteSignal<bool>),
) -> impl IntoView {
    let organization: Organization = expect_context();
    let generate_key = ServerAction::<GenerateUserAPIKeyFn>::new();
    let result = move || generate_key.value();

    Effect::watch(
        move || generate_key.version().get(),
        move |_, _, _| refresh.1.set(!refresh.0.get()),
        false,
    );

    view! {
        {move || {
            match result().get() {
                Some(res) => {
                    match res {
                        Ok(key) => {
                            view! {
                                <div class="flex flex-col gap-4">
                                    <div class="text-left">
                                        <h6>"This access key is visible only once"</h6>
                                        <p>
                                            "Be sure to copy it to your clipboard and store it securely for future use."
                                        </p>
                                    </div>
                                    <Copy>{key.key_id}_{key.key}</Copy>
                                    <div class="ms-auto">
                                        <Button
                                            variant=ButtonVariant::Secondary
                                            on_click=Box::new(move || set_show_modal.set(false))
                                        >
                                            "Close"
                                        </Button>
                                    </div>
                                </div>
                            }
                                .into_any()
                        }
                        Err(e) => view! { <div>{get_server_fn_user_error(e)}</div> }.into_any(),
                    }
                }
                None => {
                    view! {
                        <ActionForm action=generate_key>
                            <div class="max-w-sm m-auto flex flex-col gap-5">
                                <h6 class="text-left">"Generate API key"</h6>
                                <div class="inputs">
                                    <label class="flex flex-col items-start gap-2 w-100">
                                        "Name"
                                        <input
                                            type="text"
                                            name="description"
                                            class="w-full"
                                            required
                                        />
                                    </label>
                                    <input
                                        type="hidden"
                                        name="organization_id"
                                        value=organization.id
                                        required
                                    />
                                </div>
                                <div class="flex gap-4 justify-end">
                                    <Button variant=ButtonVariant::Secondary attr:r#type="submit">
                                        // disabled=DisableType::Signal(Signal::derive(generate_key.pending))
                                        "Generate Key"
                                    </Button>
                                    <Button
                                        variant=ButtonVariant::Secondary
                                        on_click=Box::new(move || set_show_modal.set(false))
                                    >
                                        "Close"
                                    </Button>
                                </div>
                            </div>
                        </ActionForm>
                    }
                        .into_any()
                }
            }
        }}
    }
}

#[component]
pub fn UserApiKey() -> impl IntoView {
    let (show_modal, set_show_modal) = signal(false);
    let organization: Organization = expect_context();
    let organization_id = organization.id;
    let (refresh_trigger, set_refresh_trigger) = signal(false);

    let remove_api_key = Action::new(|input: &(String, bool, WriteSignal<bool>)| {
        let input = input.to_owned();
        async move {
            let result = remove_user_api_key(input.0.clone()).await;
            input.2.set(!input.1);
            result
        }
    });

    let user_api_keys = Resource::new(
        move || refresh_trigger.get(),
        move |_| async move { get_user_api_keys(organization_id).await },
    );

    view! {
        <div class="flex flex-col gap-9">
            <div class="flex flex-row justify-between">
                <div class="flex flex-col gap-4">
                    <h3 class="flex gap-2 items-center">
                        <span class="material-symbols-outlined text-magenta-tint-100 !text-5xl">
                            key
                        </span>
                        "Generate API keys "
                    </h3>
                    <p>
                        "Generate an API key to access and integrate Sundry into your applications."
                    </p>
                </div>
                <div class="my-auto">
                    <Button
                        variant=ButtonVariant::Primary
                        on_click=Box::new(move || {
                            set_show_modal.set(true);
                        })
                    >
                        "Generate API Key"
                    </Button>
                </div>
            </div>
            <div class="flex flex-col divide-y divide-neutral-600">
                <Transition fallback=move || {
                    view! { "Loading API keys..." }
                }>
                    {move || {
                        user_api_keys
                            .get()
                            .map(|user_api_keys| {
                                match user_api_keys {
                                    Ok(user_api_keys) => {
                                        view! {
                                            {move || {
                                                if user_api_keys.len() == 0 {
                                                    view! {
                                                        <div>
                                                            "API keys will be shown here after you create them."
                                                        </div>
                                                    }
                                                        .into_any()
                                                } else {
                                                    user_api_keys
                                                        .iter()
                                                        .map(|user_api_key| {
                                                            let key_id = user_api_key.key_id.clone();
                                                            let (pending, _set_pending) = signal(
                                                                remove_api_key.pending().get(),
                                                            );
                                                            let refresh = set_refresh_trigger;
                                                            let refresh_state: bool = refresh_trigger.get();
                                                            view! {
                                                                <ApiCard
                                                                    variant=ApiCardVariant::APIKey
                                                                    title=user_api_key.description.to_string()
                                                                    description=format!("Connected {}", user_api_key.created_at)
                                                                    action=view! {
                                                                        <ActionTag
                                                                            pending=pending
                                                                            variant=ActionTagVariant::Remove
                                                                            on_click=ButtonAction::Action(
                                                                                Box::new(move || {
                                                                                    remove_api_key
                                                                                        .dispatch((key_id.clone(), refresh_state, refresh));
                                                                                }),
                                                                            )
                                                                        />
                                                                    }
                                                                />
                                                            }
                                                        })
                                                        .collect_view()
                                                        .into_any()
                                                }
                                            }}
                                        }
                                            .into_any()
                                    }
                                    Err(e) => view! { {get_server_fn_user_error(e)} }.into_any(),
                                }
                            })
                    }}
                </Transition>
            </div>
            <Modal show=(show_modal, set_show_modal)>
                <GenerateUserAPIKey set_show_modal refresh=(refresh_trigger, set_refresh_trigger) />
            </Modal>
        </div>
    }
}
