use crate::application::models::entities::user::Activity;
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserActivity {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub guild_id: u64,
    pub user_id: u64,
    pub date: DateTime<Utc>,
    pub activity_type: Activity,
}
