use anyhow::Result;
use chrono::{DateTime, Utc};
use mongodb::{bson::doc, options::FindOneOptions, Collection, Database};

use crate::application::models::entities::{guild::Guild, guild_name::GuildNameChange};

#[derive(Clone)]
pub struct GuildRepository {
    guilds: Collection<Guild>,
    guild_name_changes: Collection<GuildNameChange>,
}

impl GuildRepository {
    pub fn new(database: &Database) -> Self {
        Self {
            guilds: database.collection("Guilds"),
            guild_name_changes: database.collection("GuildNameChanges"),
        }
    }

    pub async fn add_guild(&self, id: u64, name: &str, date: DateTime<Utc>) -> Result<()> {
        self.update_name(id, name, date).await?;

        if self.guild_exists(id).await? {
            return Ok(());
        }

        let guild = Guild { id };
        self.guilds.insert_one(guild, None).await?;

        Ok(())
    }

    async fn guild_exists(&self, guild_id: u64) -> Result<bool> {
        Ok(self.get_guild(guild_id).await?.is_some())
    }

    async fn get_guild(&self, guild_id: u64) -> Result<Option<Guild>> {
        let filter = doc! {"id": guild_id as i64};
        Ok(self.guilds.find_one(filter, None).await?)
    }

    async fn update_name(&self, id: u64, name: &str, date: DateTime<Utc>) -> Result<()> {
        if let Some(last_name) = self.get_last_name(id).await? {
            if last_name == name {
                return Ok(());
            }
        }

        let new_name = GuildNameChange {
            guild_id: id,
            name: name.to_owned(),
            date,
        };

        self.guild_name_changes.insert_one(new_name, None).await?;

        Ok(())
    }

    async fn get_last_name(&self, guild_id: u64) -> Result<Option<String>> {
        let filter = doc! {"guild_id": guild_id as i64};
        let options = FindOneOptions::builder().sort(doc! { "date": -1 }).build();

        let possible_last_change = self.guild_name_changes.find_one(filter, options).await?;

        Ok(possible_last_change.map(|n| n.name))
    }
}
