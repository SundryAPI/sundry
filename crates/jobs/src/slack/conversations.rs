use anyhow::bail;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::handle_rate_limit;

use super::ResponseMetadata;

#[derive(Debug, Default, Serialize)]
pub(crate) struct HistoryOptionalArgs {
    cursor: Option<String>,
    include_all_metadata: Option<bool>,
    inclusive: Option<bool>,
    latest: Option<String>,
    limit: Option<usize>,
    oldest: Option<String>,
}

#[derive(Debug, Default, Serialize)]
pub(crate) struct MembersOptionalArgs {
    cursor: Option<String>,
    limit: Option<usize>,
}

pub(crate) type RepliesOptionalArgs = HistoryOptionalArgs;

#[derive(Debug, Default, Serialize)]
pub(crate) struct ListOptionalArgs {
    cursor: Option<String>,
    exclude_archived: Option<bool>,
    limit: Option<usize>,
    team_id: Option<String>,
    types: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct HistoryResponse {
    ok: bool,
    error: Option<String>,
    messages: Option<Vec<serde_json::Value>>,
    has_more: Option<bool>,
    pin_count: Option<usize>,
    response_metadata: Option<ResponseMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ListResponse {
    ok: bool,
    error: Option<String>,
    channels: Option<Vec<serde_json::Value>>,
    response_metadata: Option<ResponseMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
struct JoinResponse {
    ok: bool,
    error: Option<String>,
    channel: Option<serde_json::Value>,
    warning: Option<String>,
    response_metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MembersResponse {
    ok: bool,
    error: Option<String>,
    members: Option<Vec<serde_json::Value>>,
    response_metadata: Option<ResponseMetadata>,
}

/// See [Slack API](https://api.slack.com/methods/conversations.list)
pub(crate) async fn list(
    client: &Client,
    token: &str,
    optional_args: &ListOptionalArgs,
) -> Result<Response, reqwest::Error> {
    client
        .get("https://slack.com/api/conversations.list")
        .bearer_auth(token)
        .query(optional_args)
        .send()
        .await
}

/// See [Slack API](https://api.slack.com/methods/conversations.members)
pub(crate) async fn members(
    client: &Client,
    token: &str,
    channel: &str,
    optional_args: &MembersOptionalArgs,
) -> Result<Response, reqwest::Error> {
    client
        .get("https://slack.com/api/conversations.members")
        .bearer_auth(token)
        .query(&[("channel", channel)])
        .query(optional_args)
        .send()
        .await
}

/// See [Slack API](https://api.slack.com/methods/conversations.join)
pub(crate) async fn join(token: &str, channel: &str) -> anyhow::Result<()> {
    let client = Client::new();
    let response: JoinResponse = handle_rate_limit(|| {
        client
            .get("https://slack.com/api/conversations.join")
            .bearer_auth(token)
            .query(&[("channel", channel)])
            .send()
    })
    .await?;

    match response.error {
        Some(err) if err != "method_not_supported_for_channel_type" => bail!("{err}"),
        _ => Ok(()),
    }
}

/// See [Slack API](https://api.slack.com/methods/conversations.history)
pub(crate) async fn history(
    client: &Client,
    token: &str,
    channel: &str,
    optional_args: &HistoryOptionalArgs,
) -> Result<Response, reqwest::Error> {
    client
        .get("https://slack.com/api/conversations.history")
        .bearer_auth(token)
        .query(&[("channel", channel)])
        .query(optional_args)
        .send()
        .await
}

/// See [Slack API](https://api.slack.com/methods/conversations.replies)
pub(crate) async fn replies(
    client: &Client,
    token: &str,
    channel: &str,
    ts: &str,
    optional_args: &RepliesOptionalArgs,
) -> Result<Response, reqwest::Error> {
    client
        .get("https://slack.com/api/conversations.replies")
        .bearer_auth(token)
        .query(&[("channel", channel), ("ts", ts)])
        .query(optional_args)
        .send()
        .await
}

/// Get all channels and handle rate limiting
pub(crate) async fn list_complete(token: &str) -> anyhow::Result<Vec<serde_json::Value>> {
    let client = Client::new();

    static TYPES: &str = "public_channel,private_channel,im,mpim";

    let mut optional_args = ListOptionalArgs {
        types: Some(TYPES.to_string()),
        exclude_archived: Some(true),
        ..Default::default()
    };
    let mut channels = Vec::new();
    loop {
        let response: ListResponse =
            handle_rate_limit(|| list(&client, token, &optional_args)).await?;

        if let Some(error) = response.error {
            bail!(error)
        }

        if let Some(new_channels) = response.channels {
            channels.extend(new_channels);
        }

        match response.response_metadata {
            Some(ResponseMetadata { next_cursor }) if !next_cursor.is_empty() => {
                optional_args.cursor = Some(next_cursor);
            }
            _ => break,
        }
    }

    Ok(channels)
}

/// Get all history from the latest and handle rate limiting
pub(crate) async fn history_complete(
    token: &str,
    channel: &str,
    latest: Option<String>,
) -> anyhow::Result<Vec<serde_json::Value>> {
    let client = Client::new();

    let mut optional_args = HistoryOptionalArgs {
        latest,
        ..Default::default()
    };
    let mut messages = Vec::new();
    loop {
        let response: HistoryResponse =
            handle_rate_limit(|| history(&client, token, channel, &optional_args)).await?;

        if let Some(error) = response.error {
            bail!(error)
        }

        if let Some(mut new_messages) = response.messages {
            // TODO: if we do not add in the channel_id, we have no way of associating this
            // message back to the channel FIX: we probably need to add a column
            // to the raw table for API url args that produced the data. That is
            // where a channel_id would go
            new_messages
                .iter_mut()
                .for_each(|m| m["channel_id"] = json!(channel));
            messages.extend(new_messages);
        }

        match response.response_metadata {
            Some(ResponseMetadata { next_cursor }) if !next_cursor.is_empty() => {
                optional_args.cursor = Some(next_cursor);
            }
            _ => break,
        }
    }

    Ok(messages)
}

/// Get all history from the latest and handle rate limiting
pub(crate) async fn replies_complete(
    token: &str,
    channel: &str,
    ts: &str,
    latest: Option<String>,
) -> anyhow::Result<Vec<serde_json::Value>> {
    let client = Client::new();

    let mut optional_args = HistoryOptionalArgs {
        latest,
        ..Default::default()
    };

    let mut messages = Vec::new();

    loop {
        let response: HistoryResponse =
            handle_rate_limit(|| replies(&client, token, channel, ts, &optional_args)).await?;

        if let Some(error) = response.error {
            bail!(error)
        }

        if let Some(mut new_messages) = response.messages {
            // TODO: if we do not add in the channel_id, we have no way of associating this
            // message back to the channel FIX: we probably need to add a column
            // to the raw table for API url args that produced the data. That is
            // where a channel_id would go
            new_messages
                .iter_mut()
                .for_each(|m| m["channel_id"] = json!(channel));
            messages.extend(new_messages);
        }

        match response.response_metadata {
            Some(ResponseMetadata { next_cursor }) if !next_cursor.is_empty() => {
                optional_args.cursor = Some(next_cursor);
            }
            _ => break,
        }
    }

    Ok(messages)
}

/// Get all members of a channel and handle rate limiting
pub(crate) async fn members_complete(
    token: &str,
    channel: &str,
) -> anyhow::Result<Vec<serde_json::Value>> {
    let client = Client::new();

    let mut optional_args = MembersOptionalArgs::default();

    let mut members_vec = Vec::new();

    loop {
        let response: MembersResponse =
            handle_rate_limit(|| members(&client, token, channel, &optional_args)).await?;

        if let Some(error) = response.error {
            bail!(error)
        }

        if let Some(new_members) = response.members {
            members_vec.extend(new_members);
        }

        match response.response_metadata {
            Some(ResponseMetadata { next_cursor }) if !next_cursor.is_empty() => {
                optional_args.cursor = Some(next_cursor);
            }
            _ => break,
        }
    }

    Ok(members_vec)
}
