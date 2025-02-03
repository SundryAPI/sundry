use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use models::github::app_installation::AppInstallationMaybeWithoutOrganization;
use sqlx::{postgres::PgPoolOptions, PgPool};
use thiserror::Error;

use github_types::{CreatedEvent, WebhookEvents};
use tracing::error;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error("deserializing: {0:?}")]
    Deserializing(#[from] serde_json::Error),
    #[error("created installation missing permissions: {0:?}")]
    CreatedMissingPermissions(CreatedEvent),
    #[error("created installation missing required events: {0:?}")]
    CreatedMissingEvents(CreatedEvent),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

/// Builds a tracing subscriber from the `GITHUB_APP_LOG` environment variable
/// If the variables value is malformed or missing, sets the default log level
/// to ERROR
fn initialize_logger() {
    FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_env("GITHUB_APP_LOG"))
        .init();
}

async fn handle_webhook(pool: PgPool, payload: serde_json::Value) -> Result<(), HandlerError> {
    let webhook: WebhookEvents = serde_json::from_value(payload)?;
    match webhook {
        WebhookEvents::Issues(_) => todo!(),
        WebhookEvents::IssueComment => todo!(),
        WebhookEvents::Created(created_event) => handle_created(created_event, &pool).await?,
    }
    Ok(())
}

async fn handle_created(created_event: CreatedEvent, pool: &PgPool) -> Result<(), HandlerError> {
    // Verify permissions and events on installation
    // I am not entirely sure if it is possible that these are ever wrong though
    if !created_event.installation.permisions_meets_requirements() {
        return Err(HandlerError::CreatedMissingPermissions(created_event));
    }
    if !created_event.installation.events_meet_requirements() {
        return Err(HandlerError::CreatedMissingEvents(created_event));
    }

    let mut transaction = pool.begin().await?;

    let installation = AppInstallationMaybeWithoutOrganization::create(
        created_event.installation.id,
        &mut *transaction,
    )
    .await?;

    // If we have an organization_id the user has installed the app through our site
    // and already hit the post-callback page before we recieved this webhook.
    // We can create the task to sync it and move on.
    //
    // If we don't have an organization_id they either:
    // 1. Did not sign up through our site
    // 2. Have not landed on the post-installation page yet
    // 3. Our website is down or having some kind of issue
    //
    // We will not handle case 1. and 3. here.
    //
    // For case 2. we will create a task that periodically checks if we have an
    // organization_id for the installation and if so we will then sync the
    // installation.
    if installation.organization_id.is_some() {
    } else {
    }

    // for repository in created_event.repositories {
    //     let _repository = Repository::create(
    //         repository.id,
    //         installation.id,
    //         &repository.full_name,
    //         repository.private,
    //         &mut *transaction,
    //     )
    //     .await?;
    // }

    Ok(())
}

async fn webhook(
    State(AppState { pool, .. }): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) {
    if let Err(e) = handle_webhook(pool, payload).await {
        error!("{e:?}")
    }
}

#[tokio::main]
async fn main() {
    initialize_logger();

    let pool = PgPoolOptions::new()
        .max_connections(3)
        .idle_timeout(None)
        .connect_lazy(&std::env::var("DATABASE_URL").expect("missing `DATABASE_URL` env variable"))
        .expect("Default database URL is alformed");

    let state = AppState { pool };

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/api/webhook", post(webhook))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
