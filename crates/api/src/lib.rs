use axum::extract::Request;
use axum::routing::get;
use axum::Router;
use axum::{extract::MatchedPath, routing::post};
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing::info_span;
use tracing_subscriber::{prelude::*, EnvFilter};

pub mod model;
pub mod plan;
pub(crate) mod utils;
pub mod v1;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

pub async fn run(pool: sqlx::PgPool, port: u16) {
    tracing_subscriber::registry()
        .with(EnvFilter::from_env("API_LOG"))
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .ok();

    let state = AppState { pool };

    let app = Router::new()
        .route("/v1/context", post(v1::context))
        .route("/v1/sources", get(v1::sources))
        .with_state(state)
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                // Log the matched route's path (with placeholders not filled in).
                // Use request.uri() or OriginalUri if you want the real path.
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            }),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
