use chrono::{DateTime, Utc};

use crate::application::models::entities::user::Activity;

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
    pub guild_id: u64,
    pub nickname: String,
    pub guild_name: String,
    pub date: DateTime<Utc>,
}

pub struct UpdateNickDto {
    pub user_id: u64,
    pub guild_id: u64,
    pub new_nickname: String,
    pub date: DateTime<Utc>
}

impl From<UpdateActivityDto> for AddUserDto {
    fn from(d: UpdateActivityDto) -> Self {
        Self {
            user_id: d.user_id,
            guild_id: d.guild_id,
            nickname: d.nickname,
            guild_name: d.guild_name,
            date: d.date,
        }
    }
}