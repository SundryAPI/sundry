use std::ops::Deref;

use apalis::{postgres::PostgresStorage, prelude::*};
use models::{
    last_sync_time::LastSyncTime,
    source::{GitHubItem, RawData},
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::{types::chrono::NaiveDateTime, PgPool};

use crate::github::tokens::MANAGER;

use super::fetch_pagination;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IngestIssuesJob {
    organization_id: i64,
    repo_url: String,
    last_sync_time: Option<NaiveDateTime>,
    installation_id: i64,
}

impl Job for IngestIssuesJob {
    const NAME: &'static str = "data::github::issues";
}

pub async fn ingest(job: IngestIssuesJob, pool: Data<PgPool>) -> Result<(), Error> {
    // Generate token
    let token = MANAGER.get_token(job.installation_id).await.unwrap();

    // Query the API
    let client = Client::new();
    let url = format!("{}/issues", job.repo_url);
    let issues = fetch_pagination(&client, url, &token, job.last_sync_time)
        .await
        .unwrap();

    let urls: Vec<_> = issues
        .iter()
        .map(|issue| issue["url"].as_str().expect("issue.url").to_string())
        .collect();

    // Save into our database
    RawData::create_all(
        pool.deref(),
        job.organization_id,
        &urls,
        GitHubItem::Issues,
        &issues,
    )
    .await
    .unwrap();

    // Start other ingestion jobs
    let comment_urls: Vec<_> = issues
        .iter()
        .map(|issue| {
            issue["comments_url"]
                .as_str()
                .expect("issue.comments_url")
                .to_string()
        })
        .collect();

    super::comments::start_jobs(
        comment_urls,
        job.organization_id,
        job.installation_id,
        pool.deref(),
    )
    .await
    .unwrap();

    Ok(())
}

pub async fn start_jobs(
    repo_urls: Vec<String>,
    organization_id: i64,
    installation_id: i64,
    pool: &PgPool,
) -> Result<(), anyhow::Error> {
    let mut pg = PostgresStorage::new(pool.clone());

    let last_sync_time = LastSyncTime::get(pool, organization_id, GitHubItem::Issues).await?;

    for repo_url in repo_urls {
        pg.push(IngestIssuesJob {
            organization_id,
            installation_id,
            repo_url,
            last_sync_time,
        })
        .await?;
    }

    LastSyncTime::update(pool, organization_id, GitHubItem::Issues).await?;

    Ok(())
}
