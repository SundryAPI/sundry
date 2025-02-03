use std::{future::Future, time::Duration};

use anyhow::bail;
use apalis::prelude::*;
use reqwest::{Response, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sqlx::{
    types::chrono::{DateTime, Utc},
    PgPool,
};

pub mod github;
pub mod slack;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PollJob(DateTime<Utc>);

impl From<DateTime<Utc>> for PollJob {
    fn from(t: DateTime<Utc>) -> Self {
        Self(t)
    }
}

impl Job for PollJob {
    const NAME: &'static str = "data::poll";
}

pub async fn poll(_job: PollJob, pool: Data<PgPool>) -> Result<(), Error> {
    start_github_jobs(&pool).await.unwrap();
    start_slack_jobs(&pool).await.unwrap();
    Ok(())
}

async fn start_slack_jobs(pool: &PgPool) -> anyhow::Result<()> {
    let app_installations = models::slack::app_installation::AppInstallation::get_all(pool).await?;
    slack::users::start_jobs(app_installations.clone(), pool).await?;
    slack::channels::start_jobs(app_installations.clone(), pool).await?;
    slack::normalization::start_jobs(app_installations, pool).await?;
    Ok(())
}

async fn start_github_jobs(pool: &PgPool) -> anyhow::Result<()> {
    // reading all AppInstallations from the DB and creating an Ingest job for each
    let app_installations =
        models::github::app_installation::AppInstallation::get_all(pool).await?;
    github::repositories::start_jobs(app_installations.clone(), pool).await?;
    github::normalization::start_jobs(app_installations, pool).await?;

    Ok(())
}

async fn handle_rate_limit<F, G, T>(f: F) -> anyhow::Result<T>
where
    T: DeserializeOwned,
    F: Fn() -> G,
    G: Future<Output = Result<Response, reqwest::Error>>,
{
    const MAX_RETRIES: usize = 3;
    const DEFAULT_DELAY_SECONDS: u64 = 10;

    for _ in 0..MAX_RETRIES {
        let response = f().await?;

        if response.status() == StatusCode::TOO_MANY_REQUESTS {
            let seconds = response
                .headers()
                .get("Retry-After")
                .and_then(|s| s.to_str().ok().and_then(|s| s.parse().ok()))
                .unwrap_or(DEFAULT_DELAY_SECONDS);

            let delay = Duration::from_secs(seconds);

            tokio::time::sleep(delay).await;
        } else {
            return Ok(response.json().await?);
        }
    }

    bail!("max retries exceeded")
}
