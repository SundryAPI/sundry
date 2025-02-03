//! Slack jobs
//!
//! # Jobs
//! 1. users
//! 2. channels -> 3. channel members \-> 4. messages -> 5. replies
use serde::{Deserialize, Serialize};

pub mod channel_members;
pub mod channels;
pub mod conversations;
pub mod messages;
pub mod normalization;
pub mod replies;
pub mod users;

#[derive(Debug, Serialize, Deserialize)]
struct ResponseMetadata {
    next_cursor: String,
}
