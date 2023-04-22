use chrono::{DateTime, Utc};

pub struct CommandUseDto {
    pub user_id: u64,
    pub guild_id: Option<u64>,
    pub user_nickname: String,
    pub guild_name: String,
    pub date: DateTime<Utc>,
    pub command: String,
    pub args: String
}