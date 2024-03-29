use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NicknameChange {
    pub user_id: u64,
    pub guild_id: Option<u64>,
    pub date: DateTime<Utc>,
    pub nickname: String,
}
