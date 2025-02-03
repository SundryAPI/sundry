pub mod api;
pub mod auth;
pub mod github;
pub mod last_sync_time;
pub mod organization;
pub mod slack;
pub mod source;
pub mod user;

#[cfg(all(feature = "ssr", test))]
pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");
