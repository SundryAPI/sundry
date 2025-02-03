use std::ops::Deref;

use anyhow::Context;
use apalis::{postgres::PostgresStorage, prelude::*};
use models::{
    slack::{app_installation::AppInstallation, channel::Channel, message::Message, user::User},
    source::{RawData, SlackItem, SourceItem},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NormalizeJob {
    app_installation: AppInstallation,
}

impl Job for NormalizeJob {
    const NAME: &'static str = "data::slack::normalize";
}

pub async fn normalize(job: NormalizeJob, pool: Data<PgPool>) -> Result<(), Error> {
    let data = RawData::get_slack_unnormalized(pool.deref(), job.app_installation.organization_id)
        .await
        .unwrap();

    tracing::debug!("normalizing {} items", data.len());

    let mut messages = Vec::new();
    let mut ids = Vec::new();
    let mut channels = Vec::new();
    let mut cm_channels = Vec::new();
    let mut cm_members = Vec::new();
    let mut users = Vec::new();

    let mut transaction = pool.begin().await.unwrap();

    for raw in data {
        if let SourceItem::Slack(item) = raw.source_item {
            match item {
                SlackItem::Users => {
                    if let Ok(user) =
                        serde_json::from_value(raw.data).context("deserializing slack user")
                    {
                        users.push(user);
                    }
                }
                SlackItem::Messages => {
                    if let Ok(message) =
                        serde_json::from_value(raw.data).context("deserializing slack message")
                    {
                        messages.push(message);
                    }
                }
                SlackItem::Channels => {
                    if let Ok(channel) = serde_json::from_value::<Channel>(raw.data)
                        .context("deserializing slack channel")
                    {
                        channels.push(channel);
                    }
                }
                SlackItem::ChannelMembers => {
                    if let Ok(user_id) = serde_json::from_value::<String>(raw.data) {
                        // data_id is a concatenation of channel_id and user_id. To get the channel_id, we must strip the user_id off the end.
                        let channel_id = raw
                            .data_id
                            .split('"')
                            .next()
                            .expect("problem getting channel_id from data_id");
                        cm_channels.push(channel_id.to_string());
                        cm_members.push(user_id);
                    }
                }
            }
            ids.push(raw.id);
        }
    }

    User::insert_batch(&mut *transaction, &users).await.unwrap();

    Message::insert_batch(&mut *transaction, &messages)
        .await
        .unwrap();

    Channel::insert_batch(&mut *transaction, &channels)
        .await
        .unwrap();

    User::add_to_channel_batch(&mut *transaction, &cm_channels, &cm_members)
        .await
        .unwrap();

    RawData::mark_as_normalized_batch(&mut *transaction, &ids)
        .await
        .unwrap();

    transaction.commit().await.unwrap();

    Ok(())
}

pub async fn start_jobs(
    app_installations: Vec<AppInstallation>,
    pool: &PgPool,
) -> Result<(), anyhow::Error> {
    let mut pg = PostgresStorage::new(pool.clone());

    for app_installation in app_installations {
        pg.push(NormalizeJob { app_installation }).await?;
    }

    Ok(())
}
