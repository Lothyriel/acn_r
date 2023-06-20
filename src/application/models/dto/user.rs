use chrono::{DateTime, Utc};

use crate::application::models::entities::{user::Activity, user_activity::UserActivity};

pub struct UpdateActivityDto {
    pub user_id: u64,
    pub guild_id: u64,
    pub nickname: String,
    pub guild_name: String,
    pub activity: Activity,
    pub date: DateTime<Utc>,
}

pub struct AddUserDto {
    pub user_id: u64,
    pub guild_info: Option<GuildInfo>,
    pub nickname: String,
    pub date: DateTime<Utc>,
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

impl From<&UpdateActivityDto> for AddUserDto {
    fn from(d: &UpdateActivityDto) -> Self {
        Self {
            guild_info: Some(GuildInfo {
                guild_id: d.guild_id,
                guild_name: d.guild_name.to_owned(),
            }),
            user_id: d.user_id,
            nickname: d.nickname.to_owned(),
            date: d.date,
        }
    }
}

impl From<&UpdateActivityDto> for UserActivity {
    fn from(d: &UpdateActivityDto) -> Self {
        Self {
            guild_id: d.guild_id,
            user_id: d.user_id,
            date: d.date,
            activity_type: d.activity,
        }
    }
}
