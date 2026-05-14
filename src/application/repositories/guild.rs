use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Sqlite};

use crate::application::{
    infra::sqlite::{encode_datetime, encode_id},
    models::entities::{guild::Guild, guild_name::GuildNameChange},
};

#[derive(Clone)]
pub struct GuildRepository {
    db: Pool<Sqlite>,
}

impl GuildRepository {
    pub fn new(database: &Pool<Sqlite>) -> Self {
        Self {
            db: database.clone(),
        }
    }

    pub async fn add_guild(&self, id: u64, name: &str, date: DateTime<Utc>) -> Result<()> {
        self.update_name(id, name, date).await?;

        let guild = Guild { id };

        sqlx::query("INSERT OR IGNORE INTO guilds (id) VALUES (?)")
            .bind(encode_id(guild.id)?)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    async fn update_name(&self, id: u64, name: &str, date: DateTime<Utc>) -> Result<()> {
        if let Some(last_name) = self.get_last_name(id).await?
            && last_name == name
        {
            return Ok(());
        }

        let new_name = GuildNameChange {
            guild_id: id,
            name: name.to_owned(),
            date,
        };

        sqlx::query("INSERT INTO guild_name_changes (guild_id, date, name) VALUES (?, ?, ?)")
            .bind(encode_id(new_name.guild_id)?)
            .bind(encode_datetime(new_name.date))
            .bind(new_name.name)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    async fn get_last_name(&self, guild_id: u64) -> Result<Option<String>> {
        let possible_last_change = sqlx::query_scalar::<_, String>(
            "SELECT name FROM guild_name_changes WHERE guild_id = ? ORDER BY date DESC LIMIT 1",
        )
        .bind(encode_id(guild_id)?)
        .fetch_optional(&self.db)
        .await?;

        Ok(possible_last_change)
    }
}
