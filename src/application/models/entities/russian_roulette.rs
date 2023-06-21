use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RussianRoulette {
    pub shot: bool,
    pub number_drawn: f32,
    pub date: DateTime<Utc>,
    pub user_id: u64,
    pub guild_id: Option<u64>,
    pub command: String,
}
