use apalis::prelude::*;
use models::{
    slack::app_installation::AppInstallation,
    source::{RawData, SlackItem},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use super::conversations;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IngestChannelMembersJob {
    pub app_installation: AppInstallation,
    pub channel_id: String,
}

impl Job for IngestChannelMembersJob {
    const NAME: &'static str = "data::slack::channel_members";
}

pub async fn ingest(job: IngestChannelMembersJob, pool: Data<PgPool>) -> Result<(), Error> {
    let members =
        conversations::members_complete(&job.app_installation.access_token, &job.channel_id)
            .await
            .map_err(|e| Error::Failed(e.into()))?;

    for member in members {
        // data_id must be unique so we concatenate the channel_id and the user_id
        let data_id = format!("{}{}", job.channel_id.clone(), member.clone());
        RawData::create(
            &pool,
            job.app_installation.organization_id,
            data_id,
            SlackItem::ChannelMembers,
            member.clone(),
        )
        .await
        .map_err(|e| Error::Failed(e.into()))?;
    }

    Ok(())
}
