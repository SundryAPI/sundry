use crate::components::atoms::button::{Button, ButtonVariant, GithubSSOAuth};
use crate::components::atoms::loading::LoadingWheelCover;
use crate::components::molecules::top_nav::TopNav;
use leptos::prelude::*;

use super::sign_up::InviteParams;
use crate::utils::get_maybe_server_fn_user_error;

use leptos_router::hooks::use_query;

// Email signin process
#[server(Login, "/api")]
pub async fn login(
    email: String,
    password: String,
    invite_secret: Option<String>,
) -> Result<(), ServerFnError> {
    use super::invite::add_user_from_invitation;
    use crate::{auth::utils::login_user, error::LogAndMapInternalServerError, AppState};
    use models::user::User;

    let state: AppState = expect_context();

    let user = match User::get_with_email_password(&email, &password, &state.pool)
        .await
        .ise()?
    {
        Some(user) => user,
        None => return Err(ServerFnError::new("email or password incorrect")),
    };

    if let Some(invite_secret) = invite_secret {
        add_user_from_invitation(user.clone(), invite_secret)
            .await
            .ise()?;
    }

    login_user(&user, &state.pool).await.ise()
}

// Email signin form
#[component]
pub fn LoginEmail(
    use_sso: ReadSignal<bool>,
    set_use_sso: WriteSignal<bool>,
    set_refetch: WriteSignal<bool>,
) -> impl IntoView {
    let login = ServerAction::<Login>::new();
    let is_pending = move || login.pending().get();
    let login_value = login.value();
    let user_error = move || get_maybe_server_fn_user_error(login_value.get());

    let invite_query = use_query::<InviteParams>();

    Effect::new(move |_| {
        if login_value.get().is_some_and(|res| res.is_ok()) {
            set_refetch.set(true);
        }
    });

    view! {
        <ActionForm action=login>
            <div class="auth max-w-sm m-auto">
                <div class="inputs">
                    <label class="auth">
                        "Email"
                        <input type="email" name="email" required placeholder="awesome@email.com" />
                    </label>
                    <label class="auth">
                        "Password"
                        <input type="password" name="password" required placeholder="my password" />
                    </label>
                    {move || {
                        if let Some(invite_secret) = invite_query
                            .read()
                            .as_ref()
                            .ok()
                            .and_then(|p| p.invite_secret.clone())
                        {
                            view! {
                                <input type="hidden" name="invite_secret" value=invite_secret />
                            }
                                .into_any()
                        } else {
                            view! {}.into_any()
                        }
                    }}
                </div>
                <Button
                    variant=ButtonVariant::Primary
                    attr:r#type="submit"
                    attr:disabled=Signal::derive(is_pending)
                >
                    "Submit"
                </Button>

                {move || user_error()}

                <div>
                    <Button
                        on_click=Box::new(move || { set_use_sso.set(!use_sso.get()) })
                        variant=ButtonVariant::TertiaryWithIcon
                        attr:r#type="button"
                        attr:disabled=Signal::derive(is_pending)
                    >
                        <span class="material-symbols-outlined">arrow_back</span>
                        <p>"Back"</p>
                    </Button>
                </div>
            </div>
        </ActionForm>
    }
}

// SSO signin form
#[component]
pub fn LoginSSO(
    use_sso: ReadSignal<bool>,
    set_use_sso: WriteSignal<bool>,
    loading: ReadSignal<bool>,
    set_loading: WriteSignal<bool>,
) -> impl IntoView {
    let invite_params = use_query::<InviteParams>();
    let params = match invite_params
        .read()
        .as_ref()
        .ok()
        .and_then(|p| p.invite_secret.clone())
    {
        Some(invite) => invite.to_string(),
        None => "".to_string(),
    };

    view! {
        <div class="auth max-w-sm m-auto">
            <div class="inputs">
                <GithubSSOAuth loading set_loading data=params />

                <div class="m-auto">"Or sign in using email"</div>

                <Button
                    on_click=Box::new(move || { set_use_sso.set(!use_sso.get()) })
                    variant=ButtonVariant::Secondary
                    attr:r#type="button"
                >
                    Continue  with email
                </Button>

                <p>
                    "don't have an account: "
                    <a href=move || {
                        match invite_params
                            .read()
                            .as_ref()
                            .ok()
                            .and_then(|p| p.invite_secret.clone())
                        {
                            Some(invite_secret) => {
                                format!("/sign-up?invite_secret={}", invite_secret)
                            }
                            None => r#"/sign-up"#.to_string(),
                        }
                    }>"Sign Up"</a>
                </p>
            </div>
        </div>
    }
}

#[component]
pub fn Login(refetch_user: Box<dyn Fn()>) -> impl IntoView {
    let (use_sso, set_use_sso) = signal(true);
    let (refetch, set_refetch) = signal(false);
    let (loading, set_loading) = signal(false);

    Effect::new(move |_| {
        if refetch.get() {
            refetch_user();
        }
    });

    view! {
        <div class="pt-nav">
            <TopNav />
            <div class:hidden=move || !loading.get()>
                <LoadingWheelCover />
            </div>
            <div class="flex flex-col gap-12 pt-12">
                <div class="text-center">
                    <h2>"Try the limited beta"</h2>
                    <p>"It's free, all we ask is for your honest feedback."</p>
                </div>
                {move || {
                    if use_sso.get() {
                        view! { <LoginSSO use_sso set_use_sso loading set_loading /> }.into_any()
                    } else {
                        view! { <LoginEmail use_sso set_use_sso set_refetch /> }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
