#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use sqlx::pool::PoolOptions;
    use tracing_subscriber::{prelude::*, EnvFilter};
    use website::*;

    dotenvy::dotenv().ok();

    let pool = PoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("missing `DATABASE_URL` env variable"))
        .await
        .unwrap();

    tracing_subscriber::registry()
        .with(EnvFilter::from_env("WEBSITE_LOG"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // sqlx::migrate!("./migrations")
    //     .run(&pool)
    //     .await
    //     .expect("could not run SQLx migrations");

    let state = AppState { pool };

    let app = Router::new()
        // Any routes and state before leptos_routes are exclusively axum routes
        // Axum routes cannot use the context provided from leptos via use_context
        .route(
            "/dashboard/:organization_id/demo-ws",
            axum::routing::get(components::pages::dashboard::organization::demo::chat_endpoint),
        )
        .route(
            "/demo-ws-canned",
            axum::routing::get(
                components::pages::dashboard::organization::demo::chat_endpoint_canned,
            ),
        )
        .with_state(state.clone())
        // Leptos specific routes
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || provide_context(state.clone()),
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
