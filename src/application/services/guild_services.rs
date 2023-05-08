use anyhow::Error;
use chrono::{DateTime, Utc};
use mongodb::{bson::doc, options::FindOneOptions, Collection, Database};

use crate::application::models::entities::{guild::Guild, guild_name::GuildNameChange};

pub struct GuildServices {
    guilds: Collection<Guild>,
    guild_name_changes: Collection<GuildNameChange>,
}

impl GuildServices {
    pub fn new(database: &Database) -> Self {
        Self {
            guilds: database.collection("Guilds"),
            guild_name_changes: database.collection("GuildNameChanges"),
        }
    }

    pub async fn add_guild(&self, id: u64, name: String, date: DateTime<Utc>) -> Result<(), Error> {
        self.update_name(id, name, date).await?;

        if self.guild_exists(id).await? {
            return Ok(());
        }

        let guild = Guild { id };
        self.guilds.insert_one(guild, None).await?;

        Ok(())
    }

    async fn guild_exists(&self, guild_id: u64) -> Result<bool, Error> {
        Ok(self.get_guild(guild_id).await?.is_some())
    }

    async fn get_guild(&self, guild_id: u64) -> Result<Option<Guild>, Error> {
        let filter = doc! {"id": guild_id as i64};
        Ok(self.guilds.find_one(filter, None).await?)
    }

    async fn update_name(&self, id: u64, name: String, date: DateTime<Utc>) -> Result<(), Error> {
        match self.get_last_name(id).await? {
            Some(last_name) => {
                if last_name == name {
                    return Ok(());
                }
            }
            None => (),
        }

        let new_name = GuildNameChange {
            guild_id: id,
            name,
            date,
        };

        self.guild_name_changes.insert_one(new_name, None).await?;

        Ok(())
    }

    async fn get_last_name(&self, guild_id: u64) -> Result<Option<String>, Error> {
        let filter = doc! {"guild_id": guild_id as i64};
        let options = FindOneOptions::builder().sort(doc! { "date": -1 }).build();

        let possible_last_change = self.guild_name_changes.find_one(filter, options).await?;

        Ok(possible_last_change.map(|n| n.name))
    }
}
