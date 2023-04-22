use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct GuildNameChange {
    pub guild_id: u64,
    pub date: DateTime<Utc>,
    pub name: String,
}