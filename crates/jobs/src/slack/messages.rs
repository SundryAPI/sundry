use apalis::{postgres::PostgresStorage, prelude::*};
use models::{
    slack::{app_installation::AppInstallation, message::Message},
    source::{RawData, SlackItem},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use super::{conversations::history_complete, replies::IngestRepliesJob};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IngestMessagesJob {
    pub app_installation: AppInstallation,
    pub channel_id: String,
}

impl Job for IngestMessagesJob {
    const NAME: &'static str = "data::slack::messages";
}

pub async fn ingest(job: IngestMessagesJob, pool: Data<PgPool>) -> Result<(), Error> {
    let mut pg = PostgresStorage::new((*pool).clone());

    let latest = Message::latest(&job.channel_id, &*pool)
        .await
        .map_err(|e| Error::Failed(e.into()))?;

    let messages = history_complete(&job.app_installation.access_token, &job.channel_id, latest)
        .await
        .map_err(|e| Error::Failed(e.into()))?;

    for message in messages {
        // use the timestamp as an id because messages do not have an id
        let ts = message["ts"].as_str().unwrap().to_string();
        // https://api.slack.com/messaging/retrieving#finding_threads
        let is_threaded = message
            .as_object()
            .map(|obj| obj.contains_key("thread_ts"))
            .unwrap_or_default();

        RawData::create(
            &pool,
            job.app_installation.organization_id,
            ts.clone(),
            SlackItem::Messages,
            message,
        )
        .await
        .map_err(|e| Error::Failed(e.into()))?;

        if is_threaded {
            // get replies for this message
            pg.push(IngestRepliesJob {
                app_installation: job.app_installation.clone(),
                channel_id: job.channel_id.clone(),
                message_ts: ts,
            })
            .await
            .map_err(|e| Error::Failed(e.into()))?;
        }
    }

    Ok(())
}
