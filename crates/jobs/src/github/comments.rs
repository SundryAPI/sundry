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
pub struct IngestCommentsJob {
    organization_id: i64,
    comments_url: String,
    last_sync_time: Option<NaiveDateTime>,
    installation_id: i64,
}

impl Job for IngestCommentsJob {
    const NAME: &'static str = "data::github::comments";
}

pub async fn ingest(job: IngestCommentsJob, pool: Data<PgPool>) -> Result<(), Error> {
    // Generate token
    let token = MANAGER.get_token(job.installation_id).await.unwrap();

    // Query the API
    let client = Client::new();
    let comments = fetch_pagination(&client, job.comments_url, &token, job.last_sync_time)
        .await
        .unwrap();

    let urls: Vec<_> = comments
        .iter()
        .map(|comment| comment["url"].as_str().expect("comment.url").to_string())
        .collect();

    // Save into our database
    RawData::create_all(
        pool.deref(),
        job.organization_id,
        &urls,
        GitHubItem::Comments,
        &comments,
    )
    .await
    .unwrap();

    Ok(())
}

pub async fn start_jobs(
    comment_urls: Vec<String>,
    organization_id: i64,
    installation_id: i64,
    pool: &PgPool,
) -> Result<(), anyhow::Error> {
    let mut pg = PostgresStorage::new(pool.clone());

    let last_sync_time = LastSyncTime::get(pool, organization_id, GitHubItem::Comments).await?;

    for comments_url in comment_urls {
        pg.push(IngestCommentsJob {
            organization_id,
            installation_id,
            comments_url,
            last_sync_time,
        })
        .await?;
    }

    LastSyncTime::update(pool, organization_id, GitHubItem::Comments).await?;

    Ok(())
}
