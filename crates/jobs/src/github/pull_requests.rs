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
pub struct IngestPullRequestsJob {
    organization_id: i64,
    repo_url: String,
    last_sync_time: Option<NaiveDateTime>,
    installation_id: i64,
}

impl Job for IngestPullRequestsJob {
    const NAME: &'static str = "data::github::pull_requests";
}

pub async fn ingest(job: IngestPullRequestsJob, pool: Data<PgPool>) -> Result<(), Error> {
    // Generate token
    let token = MANAGER.get_token(job.installation_id).await.unwrap();

    // Query the API
    let client = Client::new();
    let url = format!("{}/pulls", job.repo_url);
    let pull_requests = fetch_pagination(&client, url, &token, job.last_sync_time)
        .await
        .unwrap();

    let urls: Vec<_> = pull_requests
        .iter()
        .map(|pull_request| {
            pull_request["url"]
                .as_str()
                .expect("pull_request.url")
                .to_string()
        })
        .collect();

    // Save into our database
    RawData::create_all(
        pool.deref(),
        job.organization_id,
        &urls,
        GitHubItem::PullRequests,
        &pull_requests,
    )
    .await
    .unwrap();

    // Start other ingestion jobs
    let comment_urls: Vec<_> = pull_requests
        .iter()
        .map(|pull_request| {
            pull_request["comments_url"]
                .as_str()
                .expect("pull_request.comments_url")
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

    let last_sync_time = LastSyncTime::get(pool, organization_id, GitHubItem::PullRequests).await?;

    for repo_url in repo_urls {
        pg.push(IngestPullRequestsJob {
            organization_id,
            installation_id,
            repo_url,
            last_sync_time,
        })
        .await?;
    }

    LastSyncTime::update(pool, organization_id, GitHubItem::PullRequests).await?;

    Ok(())
}
