use crate::PUBLIC_GITHUB_APP_CLIENT_ID;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use serde_json::json;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Tertiary,
    TertiaryWithIcon,
    Menu,
    Other,
}

#[component]
pub fn Button(
    #[prop(default = ButtonVariant::Primary)] variant: ButtonVariant,
    #[prop(optional)] on_click: Option<Box<dyn Fn() + 'static>>,
    children: Children,
) -> impl IntoView {
    let variant_classes = match variant {
        ButtonVariant::Primary => "primary",
        ButtonVariant::Secondary => "secondary",
        ButtonVariant::Tertiary | ButtonVariant::TertiaryWithIcon => "tertiary",
        ButtonVariant::Menu => "menu",
        ButtonVariant::Other => "other",
    };

    view! {
        <button
            class=variant_classes
            on:click=move |e| {
                e.stop_propagation();
                if let Some(handler) = &on_click {
                    handler();
                }
            }
        >
            {children()}
        </button>
    }
}

#[server(SetSession)]
pub async fn set_session(invite_uuid: String) -> Result<bool, ServerFnError> {
    use crate::auth::utils::create_anonymous_session_with_data;

    let state: crate::AppState = expect_context();

    let data = json!({"org_invite_secret": invite_uuid});

    let _ = create_anonymous_session_with_data(&state.pool, data).await;

    Ok(true)
}

#[component]
pub fn GithubSSOAuth(
    loading: ReadSignal<bool>,
    set_loading: WriteSignal<bool>,
    #[prop(optional)] data: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
) -> impl IntoView {
    let set_session = Action::new(|input: &String| {
        let input = input.to_owned();
        async move { set_session(input).await }
    });

    Effect::watch(
        move || set_session.version().get(),
        move |version, _, _| {
            if *version as i32 > 0 {
                let endpoint = format!(
                    "https://github.com/login/oauth/authorize?client_id={}",
                    &*PUBLIC_GITHUB_APP_CLIENT_ID,
                );

                use_navigate()(&endpoint, Default::default());
            }
        },
        false,
    );

    view! {
        <Button
            on_click=Box::new(move || {
                match disabled {
                    Some(true) => {},
                    _ => {
                        set_loading.set(!loading.get());
                        let data = match data.to_owned() {
                            Some(data) => data,
                            None => "".to_string(),
                        };
                        set_session.dispatch(data as String);
                    }
                }
            })
            variant=ButtonVariant::Other
            attr:r#type="button"
        >
            <div class="w-full flex align-start gap-2">
                <img src="/images/github.svg" />
                "Continue with GitHub"
            </div>
        </Button>
    }
}
