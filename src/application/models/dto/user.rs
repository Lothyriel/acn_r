use chrono::{DateTime, Utc};

use crate::application::models::entities::user::Activity;

pub struct UserActivityDto {
    pub user_id: u64,
    pub guild_info: Option<GuildInfo>,
    pub nickname: String,
    pub date: DateTime<Utc>,
    pub activity: Option<Activity>,
}

pub struct GuildInfo {
    pub guild_id: u64,
    pub guild_name: String,
}

pub struct UpdateNickDto {
    pub user_id: u64,
    pub guild_id: Option<u64>,
    pub new_nickname: String,
    pub date: DateTime<Utc>,
}
