use crate::components::atoms::button::{Button, ButtonVariant};
use crate::components::atoms::copy::Copy;
use crate::components::atoms::tag::{ActionTag, ActionTagVariant, ButtonAction};
use crate::components::molecules::api_card::{ApiCard, ApiCardVariant};
use crate::components::molecules::modal::Modal;
use crate::utils::get_server_fn_user_error;
use leptos::prelude::*;
use models::api::application::Application;
use models::{
    api::application_key::{ApplicationKey, ApplicationKeyWithKey},
    organization::Organization,
};

#[server(GenerateApplication)]
async fn generate_application(
    organization_id: i64,
    name: String,
) -> Result<Application, ServerFnError> {
    use crate::{
        auth::utils::get_logged_in_user_organization_role, error::LogAndMapInternalServerError,
    };

    let state: crate::AppState = expect_context();

    // Double check the user has acceess to the organization
    let role = get_logged_in_user_organization_role(organization_id)
        .await
        .ise()?;
    if role.is_none() {
        return Err(ServerFnError::new("not authorized"));
    }

    Application::create_with_name_organization_id(&name, organization_id, &state.pool)
        .await
        .ise()
}

#[server(GetApplications)]
async fn get_applications(organization_id: i64) -> Result<Vec<Application>, ServerFnError> {
    use crate::{
        auth::utils::get_logged_in_user_organization_role, error::LogAndMapInternalServerError,
    };

    let state: crate::AppState = expect_context();

    // Double check the user has acceess to the application
    let role = get_logged_in_user_organization_role(organization_id)
        .await
        .ise()?;
    if role.is_none() {
        return Err(ServerFnError::new("not authorized"));
    }

    Application::get_with_organization_id(organization_id, &state.pool)
        .await
        .ise()
}

#[server(GetApplicationAPIKeys)]
async fn get_application_api_keys(
    application_id: i64,
) -> Result<Vec<ApplicationKey>, ServerFnError> {
    use crate::{
        auth::utils::get_logged_in_user_organization_role, error::LogAndMapInternalServerError,
    };

    let state: crate::AppState = expect_context();

    let maybe_application: Option<Application> =
        Application::get_with_id(application_id, &state.pool)
            .await
            .ise()?;

    match maybe_application {
        Some(application) => {
            // Double check the user has acceess to the application
            let role = get_logged_in_user_organization_role(application.organization_id)
                .await
                .ise()?;
            if role.is_none() {
                return Err(ServerFnError::new("not authorized"));
            }

            ApplicationKey::get_with_application_id(application_id, &state.pool)
                .await
                .ise()
        }
        None => Err(ServerFnError::new("the application does not exist")),
    }
}

#[server(GenerateApplicationAPIKeyFn)]
async fn generate_application_api_key(
    application_id: i64,
    description: String,
) -> Result<ApplicationKeyWithKey, ServerFnError> {
    use crate::{
        auth::utils::get_logged_in_user_organization_role, error::LogAndMapInternalServerError,
    };

    let state: crate::AppState = expect_context();

    let maybe_application: Option<Application> =
        Application::get_with_id(application_id, &state.pool)
            .await
            .ise()?;

    match maybe_application {
        Some(application) => {
            // Double check the user has acceess to the application
            let role = get_logged_in_user_organization_role(application.organization_id)
                .await
                .ise()?;
            if role.is_none() {
                return Err(ServerFnError::new("not authorized"));
            }

            ApplicationKeyWithKey::create_with_application_id(
                application_id,
                &description,
                &state.pool,
            )
            .await
            .ise()
        }
        None => Err(ServerFnError::new("the application does not exist")),
    }
}

#[server(RemoveApplicationAPIKey)]
async fn remove_application_api_key(key_id: String) -> Result<(), ServerFnError> {
    use crate::{
        auth::utils::get_logged_in_user_organization_role, error::LogAndMapInternalServerError,
    };
    let state: crate::AppState = expect_context();

    let maybe_app_key = ApplicationKey::get_with_key_id(&key_id, &state.pool)
        .await
        .ise()?;

    match maybe_app_key {
        Some(app_key) => {
            let maybe_application: Option<Application> =
                Application::get_with_id(app_key.application_id, &state.pool)
                    .await
                    .ise()?;

            match maybe_application {
                Some(application) => {
                    // Double check the user has acceess to the application
                    let role = get_logged_in_user_organization_role(application.organization_id)
                        .await
                        .ise()?;
                    if role.is_none() {
                        return Err(ServerFnError::new("not authorized"));
                    }

                    app_key.delete(&state.pool).await.ise()
                }
                None => Err(ServerFnError::new("application not found")),
            }
        }
        None => Err(ServerFnError::new("key not found")),
    }
}

// This is the exact same as the GenerateUserAPIKey for now but won't be later
#[component]
fn GenerateApplicationAPIKey(
    application_id: i64,
    set_show_modal: WriteSignal<bool>,
    refresh: (ReadSignal<bool>, WriteSignal<bool>),
) -> impl IntoView {
    let generate_key = ServerAction::<GenerateApplicationAPIKeyFn>::new();
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
                                        <h6>"This API key is visible only once"</h6>
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
                                        name="application_id"
                                        value=application_id
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
fn GenerateApplication(
    set_show_modal: WriteSignal<bool>,
    refresh: (ReadSignal<bool>, WriteSignal<bool>),
) -> impl IntoView {
    let organization: Organization = expect_context();
    let generate_application = ServerAction::<GenerateApplication>::new();
    let result = move || generate_application.value();

    Effect::watch(
        move || generate_application.version().get(),
        move |_, _, _| {
            refresh.1.set(!refresh.0.get());
            set_show_modal.set(false);
        },
        false,
    );

    view! {
        <ActionForm action=generate_application>
            <div class="max-w-sm m-auto flex flex-col gap-5">
                <h6 class="text-left">"Create Sundry Application"</h6>
                <div class="inputs">
                    <label class="flex flex-col items-start gap-2 w-100">
                        "Name" <input type="text" name="name" class="w-full" required />
                    </label>
                    <input type="hidden" name="organization_id" value=organization.id required />
                </div>
                <div class="flex gap-4 justify-end">
                    <Button variant=ButtonVariant::Secondary attr:r#type="submit">
                        "Create Application"
                    </Button>
                    <Button
                        variant=ButtonVariant::Secondary
                        on_click=Box::new(move || set_show_modal.set(false))
                    >
                        "Close"
                    </Button>
                </div>
                {move || {
                    result()
                        .get()
                        .map(|res| {
                            res.err()
                                .map(|e| {
                                    view! { <div>{get_server_fn_user_error(e)}</div> }
                                })
                        })
                }}
            </div>
        </ActionForm>
    }
}

#[component]
pub fn Application(application: Application) -> impl IntoView {
    let id = application.id;

    let (refresh_api_keys_trigger, set_refresh_api_keys_trigger) = signal(false);
    let (show_modal, set_show_modal) = signal(false);

    let application_api_keys = Resource::new(
        move || refresh_api_keys_trigger.get(),
        move |_| async move { get_application_api_keys(id).await },
    );

    let remove_api_key = Action::new(|input: &(String, bool, WriteSignal<bool>)| {
        let input = input.clone();
        async move {
            let result = remove_application_api_key(input.0.clone()).await;
            input.2.set(!input.1);
            result
        }
    });

    view! {
        <div class="pt-5">
            <ApiCard
                variant=ApiCardVariant::Application
                title=application.name.to_string()
                description=format!("Created {}", application.created_at)
                action=view! {
                    <Button
                        on_click=Box::new(move || set_show_modal.set(true))
                        variant=ButtonVariant::TertiaryWithIcon
                    >
                        <p>"Generate API key"</p>
                        <span class="material-symbols-outlined">arrow_forward</span>
                    </Button>
                }
            />
        </div>
        <div>
            <Suspense fallback=move || {
                view! { <div>"Loading application api keys..."</div> }
            }>
                {move || {
                    application_api_keys
                        .get()
                        .map(|application_api_keys| {
                            match application_api_keys {
                                Ok(application_api_keys) => {
                                    let application_api_keys = StoredValue::new(
                                        application_api_keys,
                                    );
                                    view! {
                                        <Show
                                            when=move || {
                                                !application_api_keys.read_value().is_empty()
                                            }
                                            fallback=move || {
                                                view! {
                                                    <div class="text-center mt-4">
                                                        "API keys will be shown here after you create them."
                                                    </div>
                                                }
                                            }
                                        >
                                            {move || {
                                                application_api_keys
                                                    .read_value()
                                                    .iter()
                                                    .map(|application_api_key| {
                                                        let key_id = application_api_key.key_id.clone();
                                                        let (pending, _set_pending) = signal(
                                                            remove_api_key.pending().get(),
                                                        );
                                                        view! {
                                                            <ApiCard
                                                                variant=ApiCardVariant::APIKey
                                                                title=application_api_key.description.to_string()
                                                                description=format!(
                                                                    "Created {}",
                                                                    application_api_key.created_at,
                                                                )
                                                                action=view! {
                                                                    <ActionTag
                                                                        pending=pending
                                                                        variant=ActionTagVariant::Remove
                                                                        on_click=ButtonAction::Action(
                                                                            Box::new(move || {
                                                                                remove_api_key
                                                                                    .dispatch((
                                                                                        key_id.clone(),
                                                                                        refresh_api_keys_trigger.get(),
                                                                                        set_refresh_api_keys_trigger,
                                                                                    ));
                                                                            }),
                                                                        )
                                                                    />
                                                                }
                                                            />
                                                        }
                                                    })
                                                    .collect_view()
                                                    .into_any()
                                            }}
                                        </Show>
                                    }
                                        .into_any()
                                }
                                Err(e) => view! { {get_server_fn_user_error(e)} }.into_any(),
                            }
                        })
                }}
            </Suspense>
        </div>
        <Modal show=(show_modal, set_show_modal)>
            <GenerateApplicationAPIKey
                application_id=id
                set_show_modal
                refresh=(refresh_api_keys_trigger, set_refresh_api_keys_trigger)
            />
        </Modal>
    }
}

#[component]
pub fn Applications(refresh_applications: ReadSignal<bool>) -> impl IntoView {
    let organization: Organization = expect_context();
    let organization_id = organization.id;

    let applications = Resource::new(
        move || refresh_applications.get(),
        move |_| async move { get_applications(organization_id).await },
    );

    view! {
        <div>
            <Suspense fallback=move || {
                view! { <div>"Loading application api keys..."</div> }
            }>
                {move || {
                    applications
                        .get()
                        .map(|apps| {
                            match apps {
                                Ok(apps) => {
                                    let apps = StoredValue::new(apps);
                                    view! {
                                        <Show
                                            when=move || { !apps.read_value().is_empty() }
                                            fallback=move || {
                                                view! {
                                                    <div class="text-center mt-4">
                                                        "Applications will be shown here after you create them"
                                                    </div>
                                                }
                                            }
                                        >
                                            {move || {
                                                apps.read_value()
                                                    .iter()
                                                    .map(|application| {
                                                        view! { <Application application=application.clone() /> }
                                                    })
                                                    .collect_view()
                                                    .into_any()
                                            }}
                                        </Show>
                                    }
                                        .into_any()
                                }
                                Err(e) => view! { {get_server_fn_user_error(e)} }.into_any(),
                            }
                        })
                }}
            </Suspense>
        </div>
    }
}

#[component]
pub fn Developer() -> impl IntoView {
    let (show_modal, set_show_modal) = signal(false);

    let (refresh_applications, set_refresh_applications) = signal(false);

    view! {
        <div class="flex flex-col gap-4 justify-center items-center">
            <h2>"Build your app"</h2>
            <p>"Create Applications that use Sundry"</p>
            <div class="flex gap-4">
                <a href="https://docs.sundry.api" target="_blank" class="btn secondary">
                    "View Docs"
                </a>
                <Button
                    variant=ButtonVariant::Primary
                    on:click=Box::new(move |_| { set_show_modal.set(true) })
                >
                    "Create Application"
                </Button>
            </div>
        </div>
        <div>
            <Applications refresh_applications=refresh_applications />
        </div>
        <Modal show=(show_modal, set_show_modal)>
            <GenerateApplication
                set_show_modal
                refresh=(refresh_applications, set_refresh_applications)
            />
        </Modal>
    }
}
