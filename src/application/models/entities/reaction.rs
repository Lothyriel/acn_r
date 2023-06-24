use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Reaction {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub date_added: DateTime<Utc>,
    pub emotion: String,
    pub guild_id: Option<u64>,
    pub creator_id: u64,
    pub filename: String,
}
