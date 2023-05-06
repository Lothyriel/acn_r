use chrono::{Utc, DateTime};

pub struct StatsDto {
    pub initial_date: DateTime<Utc>,
    pub stats: Vec<UserStats>
}

pub struct UserStats {
    pub user_id: u64,
    pub seconds_online: i64
}