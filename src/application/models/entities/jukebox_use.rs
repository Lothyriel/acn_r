use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JukeboxUse {
    pub guild_id: u64,
    pub user_id: u64,
    pub time: DateTime<Utc>,
    pub info: TrackMetadata,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrackMetadata {
    pub author: String,
    pub title: String,
    pub uri: Option<String>,
    pub seconds: u64,
}
