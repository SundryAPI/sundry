use reqwest::{Client, IntoUrl};
use serde::de::DeserializeOwned;
use serde_json::Value;
use sqlx::types::chrono::NaiveDateTime;

pub mod comments;
pub mod issues;
pub mod normalization;
pub mod pull_requests;
pub mod repositories;
pub mod tokens;

async fn fetch_pagination<U: IntoUrl>(
    client: &Client,
    url: U,
    token: &str,
    since: Option<NaiveDateTime>,
) -> Result<Vec<Value>, reqwest::Error> {
    let mut issues = Vec::new();
    let mut page = 1;

    let url = url.into_url()?;

    loop {
        let mut request = client
            .get(url.clone())
            .bearer_auth(token)
            .header("User-Agent", "sundry") // TODO: Change this
            .query(&[("page", page), ("per_page", 100)])
            .query(&[("state", "all")]);

        if let Some(since) = since {
            request = request.query(&[("since", since.format("%Y-%m-%dT%H:%M:%SZ").to_string())]);
        }

        let response = request.send().await?.json::<Vec<Value>>().await?;
        if response.is_empty() {
            break;
        }
        issues.extend(response);
        page += 1;
    }

    Ok(issues)
}

pub async fn fetch<T, U>(client: &Client, url: U, access_token: &str) -> Result<T, reqwest::Error>
where
    T: DeserializeOwned,
    U: IntoUrl,
{
    client
        .get(url)
        .bearer_auth(access_token)
        .header("User-Agent", "sundry") // TODO: Change this
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .await?
        .json::<T>()
        .await
}
