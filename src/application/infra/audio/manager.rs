use std::{
    borrow::BorrowMut,
    collections::{HashMap, VecDeque},
    sync::{Arc, RwLock},
};

use anyhow::{anyhow, Error, Result};
use poise::serenity_prelude::{GuildId, Mentionable};
use rand::seq::SliceRandom;
use reqwest::Client;
use songbird::{input::YoutubeDl, Songbird};

use crate::{
    application::models::entities::jukebox_use::TrackMetadata, extensions::serenity::Context,
};

#[derive(Clone)]
pub struct AudioManager {
    manager: Arc<InnerAudioManager>,
}

impl AudioManager {
    pub fn new(songbird: Arc<Songbird>) -> Self {
        Self {
            manager: Arc::new(InnerAudioManager {
                songbird: songbird.clone(),
                guilds: HashMap::new(),
                client: Client::new(),
            }),
        }
    }
}

struct InnerAudioManager {
    guilds: HashMap<GuildId, GuildQueue>,
    songbird: Arc<Songbird>,
    client: Client,
}

impl InnerAudioManager {
    fn remove(&self, id: GuildId) {
        self.guilds.remove(&id);
    }

    async fn add(&mut self, id: GuildId, track: TrackMetadata) -> Result<()> {
        let guild = self.get_guild(&id)?;

        let src = YoutubeDl::new(self.client.clone(), track.uri.clone()).into();

        guild.queue.push_back(track);

        let handle = self
            .songbird
            .get(id)
            .ok_or_else(|| anyhow!("Error obtaining songbird guild call"))?;

        let mut handle = handle.lock().await;

        handle.play_input(src);

        Ok(())
    }

    async fn skip(&self, id: GuildId, ctx: Context<'_>) -> Result<(), Error> {
        let message = match self.skip_track(id)? {
            Some(track) => format!("{} Skipped: {}", ctx.author().mention(), track.track),
            None => "Nothing to skip.".to_owned(),
        };

        ctx.say(message).await?;

        Ok(())
    }

    fn skip_track(&self, id: GuildId) -> Result<Option<TrackMetadata>, Error> {
        let guild = self.get_guild(&id)?;
        Ok(guild.queue.pop_front())
    }

    fn get_guild(&self, id: &GuildId) -> Result<&mut GuildQueue, Error> {
        self.guilds
            .get_mut(id)
            .ok_or_else(|| anyhow!("Guild not found in audio manager"))
    }

    fn shuffle(&mut self, id: GuildId) -> Result<()> {
        let guild = self.get_guild(&id)?;

        let now_playing = guild
            .queue
            .remove(0)
            .ok_or_else(|| anyhow!("Queue is empty!"))?;

        guild
            .queue
            .make_contiguous()
            .shuffle(rand::thread_rng().borrow_mut());

        guild.queue.push_front(now_playing);

        Ok(())
    }

    async fn stop(&mut self, id: GuildId) -> Result<()> {
        self.guilds.remove(&id);

        self.songbird.remove(id).await?;

        Ok(())
    }
}

struct GuildsQueueManager(Arc<RwLock<InnerManager>>);

//implement
//stop -- remove guild from HashMap
//play -- add music to Queue (and guild on hashmap if doenst exist)
//playlist -- play but many
//skip -- skip music from front of the Queue and play next
//queue -- show queue
//shuffle -- shuffles the queue

struct InnerManager(HashMap<GuildId, GuildQueue>);
//implement
//get queue
//get or insert queue (for play)
//remove queue (for stop)

struct GuildQueue(RwLock<VecDeque<TrackMetadata>>);
//implement
//shuffle queue
//add track
//add many track
//get queue metadata
