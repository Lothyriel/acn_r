use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use poise::serenity_prelude::UserId;
use serde::{Deserialize, Serialize};
use songbird::input::AuxMetadata;

#[derive(Serialize, Deserialize)]
pub struct JukeboxUse {
    pub guild_id: u64,
    pub user_id: u64,
    pub track_data: String,
    pub date: DateTime<Utc>,
    pub info: TrackMetadata,
}

impl JukeboxUse {
    pub fn new(guild_id: u64, user_id: u64, track: TrackMetadata) -> Self {
        Self {
            track_data: track.track.clone(),
            date: chrono::Utc::now(),
            info: track,
            guild_id,
            user_id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrackMetadata {
    pub author: String,
    pub title: String,
    pub date: String,
    pub track: String,
    pub uri: String,
    pub lenght: usize,
    pub requester: UserId,
}

impl TrackMetadata {
    pub fn new(value: AuxMetadata, requester: UserId) -> Result<Self> {
        log::error!("testando dados metadata: {:?}", value);

        Ok(TrackMetadata {
            requester,
            author: value.artist.unwrap_or_default(),
            title: value.title.unwrap_or_default(),
            date: value.date.unwrap_or_default(),
            track: value.track.unwrap_or_default(),
            uri: value
                .source_url
                .ok_or_else(|| anyhow!("Uri not present in track metadata"))?,
            lenght: value.duration.unwrap_or_default().as_millis() as usize,
        })
    }
}
