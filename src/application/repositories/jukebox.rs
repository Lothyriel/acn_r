use anyhow::Result;
use sqlx::{Pool, Sqlite};

use crate::application::{
    infra::sqlite::{encode_datetime, encode_id},
    models::entities::jukebox_use::JukeboxUse,
};

#[derive(Clone)]
pub struct JukeboxRepository {
    db: Pool<Sqlite>,
}

impl JukeboxRepository {
    pub fn new(database: &Pool<Sqlite>) -> Self {
        Self {
            db: database.clone(),
        }
    }

    pub async fn add_jukebox_use(&self, jukebox_use: JukeboxUse) -> Result<()> {
        sqlx::query(
            "INSERT INTO jukebox_uses (guild_id, user_id, time, author, title, uri, seconds) VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(encode_id(jukebox_use.guild_id)?)
        .bind(encode_id(jukebox_use.user_id)?)
        .bind(encode_datetime(jukebox_use.time))
        .bind(jukebox_use.info.author)
        .bind(jukebox_use.info.title)
        .bind(jukebox_use.info.uri)
        .bind(i64::try_from(jukebox_use.info.seconds)?)
        .execute(&self.db)
        .await?;

        Ok(())
    }
}
