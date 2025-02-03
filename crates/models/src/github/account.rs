use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use sqlx::{Decode, PgPool};

#[cfg_attr(feature = "ssr", derive(Decode))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: i32,
    pub github_id: i64,
}

#[cfg(feature = "ssr")]
impl Account {
    pub async fn account_for_github_id(
        _pool: &PgPool,
        _github_id: i64,
    ) -> Result<Option<Account>, sqlx::Error> {
        // sqlx::query_as!(
        //     Account,
        //     "SELECT * FROM github_accounts WHERE github_id = $1",
        //     github_id
        // )
        // .fetch_optional(pool)
        // .await
        todo!()
    }
}
