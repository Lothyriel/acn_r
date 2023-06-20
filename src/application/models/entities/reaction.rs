use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Reaction {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub emotion: String,
    pub guild_id: Option<u64>,
    pub user_id: u64,
    pub filename: String,
}
