use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JukeboxUse {
    pub guild_id: u64,
    pub user_id: u64,
    pub track_data: String,
    pub date: DateTime<Utc>,
    pub info: Option<TrackInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct TrackInfo {
    pub author: String,
    pub length_in_ms: u64,
    pub title: String,
    pub uri: String,
}
