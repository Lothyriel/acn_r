use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VoiceSnippet {
    pub guild_id: u64,
    pub user_id: u64,
    pub bytes: Vec<i16>,
    pub date: DateTime<Utc>,
}
