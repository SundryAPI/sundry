use std::ops::Deref;

use apalis::{postgres::PostgresStorage, prelude::*};
use models::{
    github::app_installation::AppInstallation,
    source::{GitHubItem, RawData},
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;

use crate::github::{fetch, tokens::MANAGER};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IngestReposJob {
    app_installation: AppInstallation,
}

impl Job for IngestReposJob {
    const NAME: &'static str = "data::github::repositories";
}

pub async fn ingest(job: IngestReposJob, pool: Data<PgPool>) -> Result<(), Error> {
    // Generate token
    let token = MANAGER
        .get_token(job.app_installation.installation_id)
        .await
        .unwrap();

    // Query the API
    let client = Client::new();
    let repos: Value = fetch(
        &client,
        "https://api.github.com/installation/repositories",
        &token,
    )
    .await
    .unwrap();

    let repos = repos["repositories"]
        .as_array()
        .expect("github repositories object.repositories array");

    let urls: Vec<_> = repos
        .iter()
        .map(|repo| {
            repo["url"]
                .as_str()
                .expect("object.repositories.url")
                .to_string()
        })
        .collect();

    // Save into our database
    RawData::create_all(
        pool.deref(),
        job.app_installation.organization_id,
        &urls,
        GitHubItem::Repositories,
        repos,
    )
    .await
    .unwrap();

    // Start other ingestion jobs
    super::issues::start_jobs(
        urls.clone(),
        job.app_installation.organization_id,
        job.app_installation.installation_id,
        pool.deref(),
    )
    .await
    .unwrap();

    super::pull_requests::start_jobs(
        urls.clone(),
        job.app_installation.organization_id,
        job.app_installation.installation_id,
        pool.deref(),
    )
    .await
    .unwrap();

    Ok(())
}

pub async fn start_jobs(
    app_installations: Vec<AppInstallation>,
    pool: &PgPool,
) -> Result<(), anyhow::Error> {
    let mut pg = PostgresStorage::new(pool.clone());

    for app_installation in app_installations {
        pg.push(IngestReposJob { app_installation }).await?;
    }

    Ok(())
}
