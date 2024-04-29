use std::{
    borrow::BorrowMut,
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use anyhow::{anyhow, Error, Result};
use poise::serenity_prelude::GuildId;
use rand::seq::SliceRandom;
use tokio::sync::RwLock;

use crate::{
    application::models::entities::jukebox_use::TrackMetadata, extensions::serenity::Context,
};

pub struct AudioManager {
    manager: Arc<InnerManager>,
}
//implement
//stop -- remove guild from HashMap
//play -- add music to Queue (and guild on hashmap if doenst exist)
//playlist -- play but many
//skip -- skip music from front of the Queue and play next
//queue -- show queue
//shuffle -- shuffles the queue

impl AudioManager {
    pub fn new() -> Self {
        Self {
            manager: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn remove(&self, id: GuildId) {
        let guilds = self.manager.get_mut();
        guilds.remove(&id);
    }

    pub fn add(&mut self, id: GuildId, track: TrackMetadata) -> Result<()> {
        let guild = self.get_guild(&id)?;
        guild.push_back(track);
        Ok(())
    }

    pub fn skip(&self, id: GuildId) -> Result<Option<TrackMetadata>, Error> {
        let guild = self.get_guild(&id)?;
        Ok(guild.pop_front())
    }

    pub fn shuffle(&mut self, id: GuildId) -> Result<()> {
        let guild = self.get_guild(&id)?;

        let now_playing = guild.remove(0).ok_or_else(|| anyhow!("Queue is empty!"))?;

        guild
            .make_contiguous()
            .shuffle(rand::thread_rng().borrow_mut());

        guild.push_front(now_playing);

        Ok(())
    }

    fn get_guild(&self, id: &GuildId) -> Result<&mut GuildQueue, Error> {
        let guilds = self.manager.get_mut();
        let guild = guilds.get_mut(id);
        guild.ok_or_else(|| anyhow!("Guild not found in audio manager"))
    }
}

type InnerManager = RwLock<HashMap<GuildId, GuildQueue>>;
//implement
//get queue
//get or insert queue (for play)
//remove queue (for stop)

type GuildQueue = VecDeque<TrackMetadata>;
//implement
//shuffle queue
//add track
//add many track
//get queue metadata
