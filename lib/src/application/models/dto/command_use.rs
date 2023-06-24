use chrono::{DateTime, Utc};

use crate::application::models::dto::user::GuildInfo;

pub struct CommandUseDto {
    pub user_id: u64,
    pub guild_info: Option<GuildInfo>,
    pub user_nickname: String,
    pub date: DateTime<Utc>,
    pub command: String,
    pub args: String,
}
