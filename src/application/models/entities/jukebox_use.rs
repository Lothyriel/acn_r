use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JukeboxUse {
    pub guild_id: u64,
    pub user_id: u64,
    pub date: DateTime<Utc>,
    pub track: JukeboxTrack,
}

#[derive(Serialize, Deserialize)]
pub struct JukeboxTrack {
    pub name: String,
    pub info: Option<TrackInfo>
}

#[derive(Serialize, Deserialize)]
pub struct TrackInfo {
    pub identifier: String,
    pub author: String,
    pub length: Duration,
    pub position: u64,
    pub title: String,
    pub uri: String,
}
