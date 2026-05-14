use anyhow::Result;
use sqlx::{Pool, Sqlite};

use crate::application::{
    infra::sqlite::{encode_bool, encode_datetime, encode_id, encode_optional_id},
    models::entities::russian_roulette::RussianRoulette,
};

#[derive(Clone)]
pub struct StatsRepository {
    db: Pool<Sqlite>,
}

impl StatsRepository {
    pub fn new(database: &Pool<Sqlite>) -> Self {
        Self {
            db: database.clone(),
        }
    }

    pub async fn add_russian_roulette(&self, attempt: RussianRoulette) -> Result<()> {
        sqlx::query(
            "INSERT INTO russian_roulette (shot, number_drawn, date, user_id, guild_id, command) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(encode_bool(attempt.shot))
        .bind(f64::from(attempt.number_drawn))
        .bind(encode_datetime(attempt.date))
        .bind(encode_id(attempt.user_id)?)
        .bind(encode_optional_id(attempt.guild_id)?)
        .bind(attempt.command)
        .execute(&self.db)
        .await?;

        Ok(())
    }
}
