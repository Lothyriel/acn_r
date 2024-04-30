use std::{
    borrow::BorrowMut,
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use anyhow::{anyhow, Error, Result};
use poise::serenity_prelude::GuildId;
use rand::seq::SliceRandom;
use tokio::sync::RwLock;

use crate::application::models::entities::jukebox_use::TrackMetadata;

#[derive(Clone)]
pub struct AudioManager {
    manager: Arc<InnerManager>,
}

type InnerManager = RwLock<HashMap<GuildId, GuildQueue>>;

type GuildQueue = VecDeque<TrackMetadata>;

impl AudioManager {
    pub fn new() -> Self {
        Self {
            manager: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn remove(&self, id: GuildId) {
        let mut guilds = self.manager.write().await;

        guilds.remove(&id);
    }

    pub async fn add(&self, id: GuildId, track: TrackMetadata) -> Result<()> {
        let mut guilds = self.manager.write().await;

        let guild = guilds.entry(id).or_default();

        guild.push_back(track);

        Ok(())
    }

    pub async fn skip(&self, id: GuildId) -> Result<Option<TrackMetadata>, Error> {
        let mut guilds = self.manager.write().await;

        let guild = guilds.entry(id).or_default();

        Ok(guild.pop_front())
    }

    pub async fn shuffle(&self, id: GuildId) -> Result<()> {
        let mut guilds = self.manager.write().await;

        let guild = guilds.entry(id).or_default();

        let now_playing = guild.remove(0).ok_or_else(|| anyhow!("Queue is empty!"))?;

        guild
            .make_contiguous()
            .shuffle(rand::thread_rng().borrow_mut());

        guild.push_front(now_playing);

        Ok(())
    }

    pub async fn get_queue(&self, id: GuildId) -> Result<Vec<TrackMetadata>> {
        let mut guilds = self.manager.write().await;

        let guild = guilds.entry(id).or_default();

        let queue = guild.iter().cloned().collect();

        Ok(queue)
    }
}
