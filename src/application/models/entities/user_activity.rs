use crate::application::models::entities::user::Activity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserActivity {
    pub guild_id: u64,
    pub user_id: u64,
    pub date: DateTime<Utc>,
    pub activity_type: Activity,
}
