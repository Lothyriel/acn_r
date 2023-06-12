use chrono::{DateTime, Utc};
use lavalink_rs::model::Track;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JukeboxUse {
    pub guild_id: u64,
    pub user_id: u64,
    pub track_data: String,
    pub date: DateTime<Utc>,
    pub info: Option<TrackInfo>,
}

impl JukeboxUse {
    pub fn new(guild_id: u64, user_id: u64, track: &Track) -> Self {
        Self {
            track_data: track.track.to_owned(),
            date: chrono::Utc::now(),
            info: get_track_info(track),
            guild_id,
            user_id,
        }
    }
}

fn get_track_info(track: &Track) -> Option<TrackInfo> {
    track.info.as_ref().map(|i| TrackInfo {
        length_in_ms: i.length,
        author: i.author.to_owned(),
        title: i.title.to_owned(),
        uri: i.uri.to_owned(),
    })
}

#[derive(Serialize, Deserialize)]
pub struct TrackInfo {
    pub author: String,
    pub title: String,
    pub uri: String,
    pub length_in_ms: u64,
}
