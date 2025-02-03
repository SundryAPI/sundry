use std::str::FromStr;
use std::time::Duration;

use anyhow::Result;
use apalis::cron::{CronStream, Schedule};
use apalis::layers::retry::RetryPolicy;
use apalis::prelude::*;
use apalis::{
    layers::retry::RetryLayer, layers::tracing::TraceLayer, layers::TimeoutLayer,
    postgres::PostgresStorage,
};
use sqlx::pool::PoolOptions;
use tracing::{debug, info};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(EnvFilter::from_env("JOBS_LOG"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL").expect("Must specify path to db");

    let pool = PoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    PostgresStorage::migrations()
        .set_ignore_missing(true)
        .run(&pool)
        .await
        .expect("unable to run migrations for postgres");

    let worker_timeout = TimeoutLayer::new(Duration::from_secs(60));

    let polling_schedule = Schedule::from_str("0 * * * * *").unwrap(); // once per minute

    Monitor::<TokioExecutor>::new()
        .register({
            let pg = PostgresStorage::<jobs::PollJob>::new(pool.clone());
            WorkerBuilder::new("data-polling")
                .layer(worker_timeout.clone())
                .layer(TraceLayer::new())
                .layer(RetryLayer::new(RetryPolicy::retries(5)))
                .data(pool.clone())
                .with_storage(pg.clone())
                .stream(CronStream::new(polling_schedule).into_stream())
                .build_fn(jobs::poll)
        })
        .register({
            let pg = PostgresStorage::new(pool.clone());
            WorkerBuilder::new("slack-ingest-users")
                .layer(worker_timeout.clone())
                .layer(TraceLayer::new())
                .layer(RetryLayer::new(RetryPolicy::retries(5)))
                .data(pool.clone())
                .with_storage(pg.clone())
                .build_fn(jobs::slack::users::ingest)
        })
        .register({
            let pg = PostgresStorage::new(pool.clone());
            WorkerBuilder::new("slack-ingest-channels")
                .layer(worker_timeout.clone())
                .layer(TraceLayer::new())
                .layer(RetryLayer::new(RetryPolicy::retries(5)))
                .data(pool.clone())
                .with_storage(pg.clone())
                .build_fn(jobs::slack::channels::ingest)
        })
        .register({
            let pg = PostgresStorage::new(pool.clone());
            WorkerBuilder::new("slack-ingest-channel-members")
                .layer(worker_timeout.clone())
                .layer(TraceLayer::new())
                .layer(RetryLayer::new(RetryPolicy::retries(5)))
                .data(pool.clone())
                .with_storage(pg.clone())
                .build_fn(jobs::slack::channel_members::ingest)
        })
        .register({
            let pg = PostgresStorage::new(pool.clone());
            WorkerBuilder::new("slack-ingest-messages")
                .layer(worker_timeout.clone())
                .layer(TraceLayer::new())
                .layer(RetryLayer::new(RetryPolicy::retries(5)))
                .data(pool.clone())
                .with_storage(pg.clone())
                .build_fn(jobs::slack::messages::ingest)
        })
        .register({
            let pg = PostgresStorage::new(pool.clone());
            WorkerBuilder::new("slack-ingest-replies")
                .layer(worker_timeout.clone())
                .layer(TraceLayer::new())
                .layer(RetryLayer::new(RetryPolicy::retries(5)))
                .data(pool.clone())
                .with_storage(pg.clone())
                .build_fn(jobs::slack::replies::ingest)
        })
        .register({
            let pg = PostgresStorage::new(pool.clone());
            WorkerBuilder::new("slack-normalize")
                .layer(worker_timeout.clone())
                .layer(TraceLayer::new())
                .layer(RetryLayer::new(RetryPolicy::retries(5)))
                .data(pool.clone())
                .with_storage(pg.clone())
                .build_fn(jobs::slack::normalization::normalize)
        })
        .register({
            let pg = PostgresStorage::new(pool.clone());
            WorkerBuilder::new("github-ingest-repositories")
                .layer(worker_timeout.clone())
                .layer(TraceLayer::new())
                .layer(RetryLayer::new(RetryPolicy::retries(5)))
                .data(pool.clone())
                .with_storage(pg.clone())
                .build_fn(jobs::github::repositories::ingest)
        })
        .register({
            let pg = PostgresStorage::new(pool.clone());
            WorkerBuilder::new("github-normalize")
                .layer(worker_timeout.clone())
                .layer(TraceLayer::new())
                .layer(RetryLayer::new(RetryPolicy::retries(5)))
                .data(pool.clone())
                .with_storage(pg.clone())
                .build_fn(jobs::github::normalization::normalize)
        })
        .register({
            let pg = PostgresStorage::new(pool.clone());
            WorkerBuilder::new("github-ingest-issues")
                .layer(worker_timeout.clone())
                .layer(TraceLayer::new())
                .layer(RetryLayer::new(RetryPolicy::retries(5)))
                .data(pool.clone())
                .with_storage(pg.clone())
                .build_fn(jobs::github::issues::ingest)
        })
        .register({
            let pg = PostgresStorage::new(pool.clone());
            WorkerBuilder::new("github-ingest-pullrequests")
                .layer(worker_timeout.clone())
                .layer(TraceLayer::new())
                .layer(RetryLayer::new(RetryPolicy::retries(5)))
                .data(pool.clone())
                .with_storage(pg.clone())
                .build_fn(jobs::github::pull_requests::ingest)
        })
        .register({
            let pg = PostgresStorage::new(pool.clone());
            WorkerBuilder::new("github-ingest-comments")
                .layer(worker_timeout.clone())
                .layer(TraceLayer::new())
                .layer(RetryLayer::new(RetryPolicy::retries(5)))
                .data(pool.clone())
                .with_storage(pg.clone())
                .build_fn(jobs::github::comments::ingest)
        })
        .on_event(|e| debug!("{e:?}"))
        .shutdown_timeout(Duration::from_secs(5))
        .run_with_signal(async {
            tokio::signal::ctrl_c().await?;
            info!("Shutting down the system");
            Ok(())
        })
        .await?;
    Ok(())
}
