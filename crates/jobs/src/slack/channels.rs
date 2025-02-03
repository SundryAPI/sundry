use std::ops::Deref;

use apalis::{postgres::PostgresStorage, prelude::*};
use models::{
    slack::{app_installation::AppInstallation, channel::Channel},
    source::{RawData, SlackItem},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use super::conversations;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IngestChannelsJobs {
    app_installation: AppInstallation,
}

impl Job for IngestChannelsJobs {
    const NAME: &'static str = "data::slack::channels";
}

pub async fn ingest(job: IngestChannelsJobs, pool: Data<PgPool>) -> Result<(), Error> {
    let mut member_storage = PostgresStorage::new(pool.deref().clone());
    let mut message_storage = PostgresStorage::new(pool.deref().clone());

    let channels = conversations::list_complete(&job.app_installation.access_token)
        .await
        .map_err(|e| Error::Failed(e.into()))?;

    for channel in channels {
        let channel_id = channel["id"].as_str().unwrap().to_string();
        RawData::create(
            &pool,
            job.app_installation.organization_id,
            channel_id.clone(),
            SlackItem::Channels,
            channel.clone(),
        )
        .await
        .map_err(|e| Error::Failed(e.into()))?;

        conversations::join(&job.app_installation.access_token, &channel_id)
            .await
            .map_err(|e| Error::Failed(e.into()))?;

        Channel::add_to_app(&channel_id, job.app_installation.id, pool.deref())
            .await
            .unwrap();

        // Start channel member job
        member_storage
            .push(super::channel_members::IngestChannelMembersJob {
                app_installation: job.app_installation.clone(),
                channel_id: channel_id.clone(),
            })
            .await
            .map_err(|e| Error::Failed(e.into()))?;
        message_storage
            .push(super::messages::IngestMessagesJob {
                app_installation: job.app_installation.clone(),
                channel_id,
            })
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
        pg.push(IngestChannelsJobs { app_installation }).await?;
    }

    Ok(())
}
