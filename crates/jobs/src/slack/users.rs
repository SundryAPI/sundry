use anyhow::bail;
use apalis::{postgres::PostgresStorage, prelude::*};
use models::{
    slack::app_installation::AppInstallation,
    source::{RawData, SlackItem},
};
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::handle_rate_limit;

use super::ResponseMetadata;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IngestUsersJob {
    app_installation: AppInstallation,
}

impl Job for IngestUsersJob {
    const NAME: &'static str = "data::slack::users";
}

pub async fn ingest(job: IngestUsersJob, pool: Data<PgPool>) -> Result<(), Error> {
    let members = list_complete(&job.app_installation.access_token)
        .await
        .map_err(|e| Error::Failed(e.into()))?;

    for user in members {
        let id = user["id"].as_str().unwrap().to_string();
        RawData::create(
            &pool,
            job.app_installation.organization_id,
            id,
            SlackItem::Users,
            user.clone(),
        )
        .await
        .map_err(|e| Error::Failed(e.into()))?;
    }

    Ok(())
}

pub async fn start_jobs(
    app_installations: Vec<AppInstallation>,
    pool: &PgPool,
) -> Result<(), anyhow::Error> {
    let mut pg = PostgresStorage::new(pool.clone());

    for app_installation in app_installations {
        pg.push(IngestUsersJob { app_installation }).await?;
    }

    Ok(())
}

#[derive(Debug, Default, Serialize)]
struct ListOptionalArgs {
    cursor: Option<String>,
    include_locale: Option<bool>,
    limit: Option<usize>,
    team_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ListResponse {
    ok: bool,
    error: Option<String>,
    members: Option<Vec<serde_json::Value>>,
    cache_ts: Option<usize>,
    response_metadata: Option<ResponseMetadata>,
}

/// See [Slack API](https://api.slack.com/methods/users.list)
async fn list(
    client: &Client,
    token: &str,
    optional_args: &ListOptionalArgs,
) -> Result<Response, reqwest::Error> {
    client
        .get("https://slack.com/api/users.list")
        .bearer_auth(token)
        .query(optional_args)
        .send()
        .await
}

/// Get all channels and handle rate limiting
async fn list_complete(token: &str) -> anyhow::Result<Vec<serde_json::Value>> {
    let client = Client::new();

    let mut optional_args = ListOptionalArgs::default();
    let mut members = Vec::new();
    loop {
        let response: ListResponse =
            handle_rate_limit(|| list(&client, token, &optional_args)).await?;

        if let Some(error) = response.error {
            bail!(error)
        }

        if let Some(new_members) = response.members {
            members.extend(new_members);
        }

        match response.response_metadata {
            Some(ResponseMetadata { next_cursor }) if !next_cursor.is_empty() => {
                optional_args.cursor = Some(next_cursor);
            }
            _ => break,
        }
    }

    Ok(members)
}
