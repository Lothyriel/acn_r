use std::sync::Arc;

use anyhow::{anyhow, bail, Result};
use log::error;
use poise::serenity_prelude::{
    async_trait, ChannelId, GuildId, Mentionable, MessageBuilder, UserId,
};
use reqwest::Client;
use songbird::{
    input::{AuxMetadata, Compose, YoutubeDl},
    tracks::{TrackHandle, TrackState},
    Event, EventContext, EventHandler, Songbird, TrackEvent,
};

use crate::{
    application::{
        models::entities::jukebox_use::{JukeboxUse, TrackMetadata},
        repositories::jukebox::JukeboxRepository,
    },
    extensions::{
        log_ext::LogExt,
        serenity::{context_ext::ContextExt, Context},
    },
};

use super::manager::AudioManager;

#[derive(Clone)]
struct SongbirdEventHandler {
    guild_id: GuildId,
    songbird: Arc<Songbird>,
}

#[async_trait]
impl EventHandler for SongbirdEventHandler {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        match ctx {
            EventContext::Track(t) => self.track_finish_handler(t, self.guild_id).await.log(),

            e => log::warn!("This happened {:?}", e),
        };

        None
    }
}

impl SongbirdEventHandler {
    fn new(guild_id: GuildId, songbird: Arc<Songbird>) -> Self {
        Self { guild_id, songbird }
    }

    async fn track_finish_handler(
        &self,
        _tracks_info: &[(&TrackState, &TrackHandle)],
        guild_id: GuildId,
    ) -> Result<()> {
        let empty = { true };

        if empty {
            self.songbird.remove(guild_id).await?;
        }

        Ok(())
    }
}

pub struct AudioPlayer {
    guild_id: GuildId,
    user_id: UserId,
    jukebox_repository: JukeboxRepository,
    http_client: Client,
    manager: AudioManager,
    songbird: Arc<Songbird>,
}

impl AudioPlayer {
    pub fn new(
        guild_id: GuildId,
        user_id: UserId,
        manager: AudioManager,
        jukebox_repository: JukeboxRepository,
        songbird: Arc<Songbird>,
    ) -> Self {
        Self {
            manager,
            http_client: Client::new(),
            guild_id,
            user_id,
            jukebox_repository,
            songbird,
        }
    }

    pub async fn shuffle(&self, ctx: Context<'_>) -> Result<()> {
        ctx.say("Shuffled queue!").await?;

        Ok(())
    }

    pub async fn stop(&self, ctx: Context<'_>) -> Result<()> {
        self.stop_player().await?;

        ctx.say("Player stopped! Queue cleared!").await?;

        Ok(())
    }

    pub async fn skip(&self, ctx: Context<'_>) -> Result<()> {
        let message = match self.manager.skip(self.guild_id).await? {
            Some(track) => format!("{} Skipped: {}", ctx.author().mention(), track.track),
            None => "Nothing to skip.".to_owned(),
        };

        ctx.say(message).await?;

        Ok(())
    }

    pub async fn show_queue(&self, ctx: Context<'_>) -> Result<()> {
        let queue_description = {
            let mut message_builder = MessageBuilder::new();

            let queue = self.manager.get_queue(self.guild_id).await?;

            match queue.is_empty() {
                false => {
                    message_builder.push_line("Queue: ");

                    for (i, track) in queue.iter().take(10).enumerate() {
                        let track_name = &track.track;

                        let now = if i == usize::MIN { "▶️" } else { "" };

                        let line = format!("- {} {} | By: {}", now, track_name, track.requester);

                        message_builder.push_line(line);
                    }

                    if queue.len() > 10 {
                        message_builder.push(format!("{} more tracks...", queue.len() - 10));
                    }
                }
                true => {
                    message_builder.push_line("EMPTY!!!");
                }
            };

            message_builder.build()
        };

        ctx.say(queue_description).await?;

        Ok(())
    }

    pub async fn play(&self, ctx: Context<'_>, query: String) -> Result<()> {
        self.assure_connected(ctx).await?;

        self.queue_music(ctx, query).await
    }

    pub async fn playlist(&self, ctx: Context<'_>, query: String) -> Result<()> {
        self.assure_connected(ctx).await?;

        self.queue_playlist(ctx, query).await
    }

    pub async fn join_voice_channel(&self, channel_id: ChannelId) -> Result<()> {
        let handle = match self.songbird.join(self.guild_id, channel_id).await {
            Ok(h) => h,
            Err(error) => bail!(
                "Guild {} | Error joining the channel: {}",
                self.guild_id,
                error
            ),
        };

        let mut handle = handle.lock().await;

        handle.add_global_event(
            Event::Track(TrackEvent::End),
            SongbirdEventHandler::new(self.guild_id, self.songbird.clone()),
        );

        Ok(())
    }

    async fn assure_connected(&self, ctx: Context<'_>) -> Result<()> {
        let channel = match ctx.assure_connected().await? {
            Some(c) => c,
            None => {
                ctx.say("Please join a voice channel.").await?;
                return Ok(());
            }
        };

        let should_join = match self.songbird.get(self.guild_id) {
            Some(call) => {
                let guard = call.lock().await;

                match guard.current_connection() {
                    Some(current_connection) => {
                        current_connection.channel_id.map(|c| c.0.get()) != Some(channel.get())
                    }
                    None => true,
                }
            }
            None => true,
        };

        if should_join {
            self.join_voice_channel(channel).await?
        }

        Ok(())
    }

    pub async fn stop_player(&self) -> Result<()> {
        self.manager.remove(self.guild_id).await;

        Ok(())
    }

    async fn queue_music(&self, ctx: Context<'_>, query: String) -> Result<()> {
        let mut src = YoutubeDl::new_search(self.http_client.clone(), query);

        match src.aux_metadata().await {
            Ok(metadata) => self.add_to_queue(ctx, metadata).await,
            Err(e) => {
                error!("{}", e);

                ctx.say("Could not find any video of the search query.")
                    .await?;

                Ok(())
            }
        }
    }

    async fn queue_playlist(&self, ctx: Context<'_>, query: String) -> Result<()> {
        let src = YoutubeDl::new_search(self.http_client.clone(), query)
            .search(Some(10))
            .await?;

        let tracks_count = src.len();

        for track in src.into_iter() {
            self.add_track_to_queue(track).await?;
        }

        let reply = format!("Added {} tracks to the queue", tracks_count);

        ctx.say(reply).await?;

        Ok(())
    }

    async fn add_to_queue(&self, ctx: Context<'_>, metadata: AuxMetadata) -> Result<()> {
        let message = format!("Added to queue: {:?}", metadata);

        self.add_track_to_queue(metadata).await?;

        ctx.say(message).await?;

        Ok(())
    }

    async fn add_track_to_queue(&self, metadata: AuxMetadata) -> Result<()> {
        let track = TrackMetadata::new(metadata.clone(), self.user_id)?;

        let handle = self
            .songbird
            .get(self.guild_id)
            .ok_or_else(|| anyhow!("Error obtaining songbird guild call"))?;

        let mut handle = handle.lock().await;

        handle.play_input(YoutubeDl::new(self.http_client.clone(), track.uri.clone()).into());

        self.insert_jukebox_use(&track);

        self.manager.add(self.guild_id, track).await
    }

    fn insert_jukebox_use(&self, track: &TrackMetadata) {
        let service = self.jukebox_repository.to_owned();

        let j_use = JukeboxUse::new(self.guild_id.get(), self.user_id.get(), track.clone());

        tokio::spawn(async move { service.add_jukebox_use(j_use).await.log() });
    }
}
