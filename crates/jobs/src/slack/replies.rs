use apalis::prelude::*;
use models::{
    slack::app_installation::AppInstallation,
    source::{RawData, SlackItem},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use super::conversations::replies_complete;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IngestRepliesJob {
    pub app_installation: AppInstallation,
    pub channel_id: String,
    pub message_ts: String,
}

impl Job for IngestRepliesJob {
    const NAME: &'static str = "data::slack::replies";
}

pub async fn ingest(job: IngestRepliesJob, pool: Data<PgPool>) -> Result<(), Error> {
    let messages = replies_complete(
        &job.app_installation.access_token,
        &job.channel_id,
        &job.message_ts,
        None,
    )
    .await
    .map_err(|e| Error::Failed(e.into()))?;

    for message in messages {
        // use the timestamp as an id because messages do not have an id
        let ts = message["ts"].as_str().unwrap().to_string();
        RawData::create(
            &pool,
            job.app_installation.organization_id,
            ts.clone(),
            SlackItem::Messages,
            message,
        )
        .await
        .map_err(|e| Error::Failed(e.into()))?;
    }
    Ok(())
}
