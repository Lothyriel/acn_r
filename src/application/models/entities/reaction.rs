use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Reaction {
    #[serde(rename = "_id")]
    pub id: String,
    pub date_added: DateTime<Utc>,
    pub emotion: String,
    pub guild_id: u64,
    pub creator_id: u64,
    pub filename: String,
}

#[derive(Serialize, Deserialize)]
pub struct ReactionUse {
    pub reaction_id: String,
    pub date: DateTime<Utc>,
    pub user_id: u64,
}
