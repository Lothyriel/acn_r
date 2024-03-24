use std::{
    borrow::BorrowMut,
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use anyhow::{anyhow, Error, Result};
use poise::serenity_prelude::GuildId;
use rand::seq::SliceRandom;
use reqwest::Client;
use songbird::{
    input::{AuxMetadata, YoutubeDl},
    Songbird,
};
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::{
    application::models::entities::jukebox_use::TrackMetadata, extensions::log_ext::LogExt,
};

#[derive(Clone)]
pub struct AudioManager {
    manager: Arc<InnerAudioManager>,
}

impl AudioManager {
    pub fn new(songbird: Arc<Songbird>) -> Self {
        let (sender, receiver) = mpsc::channel(100);

        Self {
            manager: Arc::new(InnerAudioManager {
                songbird: songbird.clone(),
                guilds: HashMap::new(),
                sender,
                receiver,
            }),
        }
    }

    pub fn get_sender(&self) -> Sender<Message> {
        self.manager.sender.clone()
    }

    pub fn start(&self) {
        tokio::spawn(async move {
            loop {
                match self.manager.receiver.recv().await {
                    Some(m) => self.manager.handle_message(m).await.log(),
                    None => break log::error!("Channel is closed??"),
                }
            }
        });
    }
}

struct InnerAudioManager {
    guilds: HashMap<GuildId, GuildAudio>,
    songbird: Arc<Songbird>,
    sender: Sender<Message>,
    receiver: Receiver<Message>,
    client: Client,
}

pub struct Message {
    action: Action,
    guild_id: GuildId,
}

impl Message {
    pub fn new(action: Action, guild_id: GuildId) -> Self {
        Self { action, guild_id }
    }
}

pub enum Action {
    RemoveGuild,
    AddTrack(TrackMetadata),
    SkipTrack,
    ShufflePlaylist,
}

impl InnerAudioManager {
    async fn handle_message(&mut self, message: Message) -> Result<()> {
        let id = message.guild_id;

        match message.action {
            Action::RemoveGuild => self.remove(id),
            Action::AddTrack(t) => self.add(id, t).await?,
            Action::SkipTrack => self.skip(id)?,
            Action::ShufflePlaylist => self.shuffle(id)?,
        };

        Ok(())
    }

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

    fn skip(&self, id: GuildId) -> Result<(), Error> {
        let guild = self.get_guild(&id)?;

        guild.queue.pop_front();

        Ok(())
    }

    fn get_guild(&self, id: &GuildId) -> Result<&mut GuildAudio, Error> {
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
}

struct GuildAudio {
    queue: VecDeque<TrackMetadata>,
}
