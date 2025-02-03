use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::hooks::use_navigate;
use leptos_router::{
    components::{Outlet, ParentRoute, ProtectedRoute, Redirect, Route, Router, Routes},
    path,
};
use std::sync::Arc;

pub mod auth;
pub mod components;
pub mod email;
pub mod error;
pub mod error_template;
pub mod user;
pub mod utils;

use crate::{
    auth::utils::get_logged_in_user,
    components::pages::{
        auth::{Login, SignUp},
        dashboard, home,
    },
    error_template::{AppError, ErrorTemplate},
    user::User,
};

// We expose PUBLIC variables to the client so we need to know them at compile
// time
pub static PUBLIC_DEV_MODE: std::sync::LazyLock<bool> =
    std::sync::LazyLock::new(|| matches!(std::option_env!("PUBLIC_DEV_MODE"), Some("true")));

pub static PUBLIC_GITHUB_APP_NAME: std::sync::LazyLock<&'static str> =
    std::sync::LazyLock::new(|| std::env!("PUBLIC_GITHUB_APP_NAME"));

pub static PUBLIC_GITHUB_APP_CLIENT_ID: std::sync::LazyLock<&'static str> =
    std::sync::LazyLock::new(|| std::env!("PUBLIC_GITHUB_APP_CLIENT_ID"));

pub static PUBLIC_SLACK_CLIENT_ID: std::sync::LazyLock<&'static str> =
    std::sync::LazyLock::new(|| std::env!("PUBLIC_SLACK_CLIENT_ID"));

pub static PUBLIC_SLACK_SCOPES: std::sync::LazyLock<&'static str> =
    std::sync::LazyLock::new(|| std::env!("PUBLIC_SLACK_SCOPES"));

pub static PUBLIC_SLACK_USER_SCOPES: std::sync::LazyLock<&'static str> =
    std::sync::LazyLock::new(|| std::env!("PUBLIC_SLACK_USER_SCOPES"));

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::App;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}

#[cfg(feature = "ssr")]
#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::Pool<sqlx::Postgres>,
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <link rel="preconnect" href="https://fonts.googleapis.com" />
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
                <link
                    href="https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:ital,wght@0,100;0,200;0,300;0,400;0,500;0,600;0,700;1,100;1,200;1,300;1,400;1,500;1,600;1,700&display=swap"
                    rel="stylesheet"
                />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn UserGate(
    user_resource: Resource<Result<Option<models::user::User>, ServerFnError>>,
) -> impl IntoView {
    // Currently using workaround: https://github.com/leptos-rs/leptos/issues/3042
    let outer_owner = Owner::current().unwrap();
    let inner_view = move || {
        Suspend::new({
            let outer_owner = outer_owner.clone();
            async move {
                match user_resource.await.ok().flatten() {
                    Some(user) => {
                        let user = User::new(user, Arc::new(move || user_resource.refetch()));
                        outer_owner.with(|| {
                            provide_context(user);
                        });
                        Outlet()
                    }
                    .into_any(),
                    None => view! { <Redirect path="/sign-in" /> }.into_any(),
                }
            }
        })
    };

    // Using Transition workaround: https://github.com/leptos-rs/leptos/issues/2907
    view! {
        <Transition fallback=move || {
            view! { <p>"Loading User..."</p> }
        }>{inner_view}</Transition>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (reroute_path, set_reroute_path) = signal::<Option<String>>(None);
    let user_resource = Resource::new(
        move || (),
        move |_| async move {
            let user = get_logged_in_user().await;
            if let Some(reroute_path) = reroute_path.get_untracked() {
                set_reroute_path.set(None);
                use_navigate()(&reroute_path, Default::default());
            }
            user
        },
    );
    let refetch_user = move || user_resource.refetch();

    let (_is_routing, set_is_routing) = signal(false);

    view! {
        <Stylesheet id="leptos" href="/pkg/website.css" />

        // Google material icons
        <Link
            rel="stylesheet"
            href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200"
        />

        // Link our font
        // <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        // <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin=""/>
        // <Link href="https://fonts.googleapis.com/css2?family=Montserrat:ital,wght@0,100..900;1,100..900&display=swap" rel="stylesheet"/>

        // Set some meta tags
        <Title text="Sundry" />
        <Meta name="description" content="" />
        <Link rel="apple-touch-icon" sizes="180x180" href="/favicon/apple-touch-icon.png" />
        <Link rel="icon" sizes="32x32" href="/favicon/favicon-32x32.png" />
        <Link rel="icon" sizes="16x16" href="/favicon/favicon-16x16.png" />
        // <Link rel="manifest" href="/site.webmanifest"/>

        <Router set_is_routing>
            // shows a progress bar while async data is loading
            // <div class="routing-progress">
            // <RoutingProgress is_routing max_time=Duration::from_millis(250) />
            // </div>

            <main>
                <Routes fallback=|| {
                    let mut outside_errors = Errors::default();
                    outside_errors.insert_with_default_key(AppError::NotFound);
                    view! { <ErrorTemplate outside_errors /> }.into_view()
                }>
                    <Route path=path!("") view=home::Home />

                    <components::pages::playground::routes::Routes />

                    <ParentRoute
                        path=path!("/dashboard")
                        view=move || view! { <UserGate user_resource /> }
                    >
                        <dashboard::dashboard::Routes />
                    </ParentRoute>

                    <ProtectedRoute
                        path=path!("sign-in")
                        condition=move || {
                            user_resource
                                .get()
                                .map(|maybe_user| maybe_user.ok().flatten().is_none())
                        }
                        redirect_path=|| "/dashboard"
                        view=move || {
                            view! { <Login refetch_user=Box::new(refetch_user) /> }
                        }
                    />

                    <ProtectedRoute
                        path=path!("sign-up")
                        condition=move || {
                            user_resource
                                .get()
                                .map(|maybe_user| maybe_user.ok().flatten().is_none())
                        }
                        redirect_path=|| "/dashboard"
                        view=move || {
                            view! { <SignUp refetch_user=Box::new(refetch_user) /> }
                        }
                    />

                    <Route
                        path=path!("/github/auth-callback")
                        view=move || {
                            view! {
                                <components::pages::auth::github::callback::AuthCallback refetch_user=Box::new(move |
                                    path: Option<String>|
                                {
                                    set_reroute_path.set(path);
                                    user_resource.refetch()
                                }) />
                            }
                        }
                    />

                    <Route
                        path=path!("/slack/auth-callback")
                        view=move || {
                            view! {
                                <components::pages::auth::slack::callback::AuthCallback refetch_user=Box::new(move |
                                    path: Option<String>|
                                {
                                    set_reroute_path.set(path);
                                    user_resource.refetch()
                                }) />
                            }
                        }
                    />
                </Routes>
            </main>
        </Router>
    }
}
