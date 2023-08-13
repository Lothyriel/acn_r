use chrono::{DateTime, Utc};
use mongodb::bson::Binary;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VoiceSnippet {
    pub guild_id: u64,
    pub user_id: u64,
    pub voice_data: Binary,
    pub date: DateTime<Utc>,
}
