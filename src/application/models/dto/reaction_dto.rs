use std::io::Cursor;

use chrono::{DateTime, Utc};

pub struct AddReactionDto {
    pub date: DateTime<Utc>,
    pub emotion: String,
    pub guild_id: Option<u64>,
    pub user_id: u64,
    pub filename: String,
    pub bytes: Cursor<Vec<u8>>,
}
