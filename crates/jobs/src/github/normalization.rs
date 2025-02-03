use std::ops::Deref;

use apalis::{postgres::PostgresStorage, prelude::*};
use models::{
    github::{
        app_installation::AppInstallation, comment::Comment, issue::Issue,
        pull_request::PullRequest, repository::Repository,
    },
    source::{GitHubItem, RawData, SourceItem},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NormalizeJob {
    app_installation: AppInstallation,
}

impl Job for NormalizeJob {
    const NAME: &'static str = "data::github::normalize";
}

pub async fn normalize(job: NormalizeJob, pool: Data<PgPool>) -> Result<(), Error> {
    let data = RawData::get_github_unnormalized(pool.deref(), job.app_installation.organization_id)
        .await
        .unwrap();

    for raw in data {
        let mut transaction = pool.begin().await.unwrap();
        match raw.source_item {
            SourceItem::GitHub(GitHubItem::Issues) => {
                let issue: Issue = serde_json::from_value(raw.data).unwrap();
                if !issue.insert(&mut transaction).await.unwrap() {
                    transaction.rollback().await.unwrap();
                    continue;
                }
            }
            SourceItem::GitHub(GitHubItem::PullRequests) => {
                let pull_request: PullRequest = serde_json::from_value(raw.data).unwrap();
                if !pull_request.insert(&mut transaction).await.unwrap() {
                    transaction.rollback().await.unwrap();
                    continue;
                }
            }
            SourceItem::GitHub(GitHubItem::Comments) => {
                let comment: Comment = serde_json::from_value(raw.data).unwrap();
                if !comment.insert(&mut transaction).await.unwrap() {
                    transaction.rollback().await.unwrap();
                    continue;
                }
            }
            SourceItem::GitHub(GitHubItem::Repositories) => {
                let repository: Repository = serde_json::from_value(raw.data).unwrap();
                repository
                    .insert(job.app_installation.id, &mut transaction)
                    .await
                    .unwrap();
            }
            SourceItem::Slack(_) => todo!(),
            SourceItem::Jira(_) => todo!(),
            SourceItem::Google(_) => todo!(),
        }

        RawData::mark_as_normalized(raw.id, &mut transaction)
            .await
            .unwrap();
        transaction.commit().await.unwrap();
    }

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
