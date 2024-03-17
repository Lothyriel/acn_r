use std::{borrow::BorrowMut, sync::Arc};

use anyhow::{anyhow, bail, Error};
use poise::serenity_prelude::{ChannelId, Mentionable, MessageBuilder};
use rand::seq::SliceRandom;
use reqwest::Client;
use songbird::{
    input::{AuxMetadata, YoutubeDl},
    tracks::{TrackHandle, TrackState},
    Event, EventContext, EventHandler, Songbird, TrackEvent,
};

use crate::{
    application::{
        models::entities::jukebox_use::JukeboxUse, repositories::jukebox::JukeboxRepository,
    },
    extensions::{
        log_ext::LogExt,
        serenity::{context_ext::ContextExt, Context},
    },
};

struct SongbirdEventHandler {
    guild_id: u64,
    songbird: Arc<Songbird>,
}

impl EventHandler for SongbirdEventHandler {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        match ctx {
            EventContext::Track(t) => self.track_finish_handler(t, self.guild_id).await.log(),
            e => log::warn!("This happened {}", e),
        };

        None
    }
}

impl SongbirdEventHandler {
    fn new(guild_id: u64, songbird: Arc<Songbird>) -> Self {
        Self { guild_id, songbird }
    }

    async fn track_finish_handler(
        &self,
        tracks_info: &[(&TrackState, &TrackHandle)],
        guild_id: u64,
    ) -> Result<(), Error> {
        let empty = { todo!("get guild queue and see if its empty") };

        if empty {
            self.songbird.remove(guild_id).await?;
        }

        Ok(())
    }
}

pub struct AudioPlayer {
    guild_id: u64,
    user_id: u64,
    songbird: Arc<Songbird>,
    jukebox_repository: JukeboxRepository,
    http_client: Client,
}

impl AudioPlayer {
    pub fn new(
        guild_id: u64,
        user_id: u64,
        songbird: Arc<Songbird>,
        jukebox_repository: JukeboxRepository,
        http_client: Client,
    ) -> Self {
        Self {
            http_client,
            guild_id,
            user_id,
            songbird,
            jukebox_repository,
        }
    }

    pub async fn shuffle(&self, ctx: Context<'_>) -> Result<(), Error> {
        self.shuffle_playlist().await?;

        ctx.say("Shuffled queue!").await?;

        Ok(())
    }

    pub async fn stop(&self, ctx: Context<'_>) -> Result<(), Error> {
        self.stop_player().await?;

        ctx.say("Player stopped! Queue cleared!").await?;

        Ok(())
    }

    pub async fn skip(&self, ctx: Context<'_>) -> Result<(), Error> {
        let message = match self.skip_track().await? {
            Some(track) => format!(
                "{} Skipped: {}",
                ctx.author().mention(),
                get_track_name(&track.track)
            ),
            None => "Nothing to skip.".to_owned(),
        };

        ctx.say(message).await?;

        Ok(())
    }

    pub async fn show_queue(&self, ctx: Context<'_>) -> Result<(), Error> {
        let queue_description = {
            let nodes = self.lava_client.nodes().await;

            let node = nodes
                .get(&self.guild_id)
                .ok_or_else(|| anyhow!("[Queue] Couldn't get node for {}", self.guild_id))?;

            let mut message_builder = MessageBuilder::new();

            match node.queue.is_empty() {
                false => {
                    message_builder.push_line("Queue: ");

                    for (i, track) in node.queue.iter().take(10).enumerate() {
                        let track_name = get_track_name(&track.track);

                        let requester = track
                            .requester
                            .map(|r| format!("<@{}>", r.0))
                            .unwrap_or_else(|| "Unknown".to_owned());

                        let now = if i == usize::MIN { "▶️" } else { "" };

                        let line = format!("- {} {} | By: {}", now, track_name, requester);

                        message_builder.push_line(line);
                    }

                    if node.queue.len() > 10 {
                        message_builder.push(format!("{} more tracks...", node.queue.len() - 10));
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

    pub async fn play(&self, ctx: Context<'_>, query: String) -> Result<(), Error> {
        self.assure_connected(ctx).await?;

        self.queue_music(ctx, query).await
    }

    pub async fn playlist(&self, ctx: Context<'_>, query: String) -> Result<(), Error> {
        self.assure_connected(ctx).await?;

        self.queue_playlist(ctx, query).await
    }

    pub async fn join_voice_channel(&self, channel_id: ChannelId) -> Result<(), Error> {
        let handle = match self.songbird.join(self.guild_id, channel_id).await {
            Ok(h) => Ok(h),
            Err(error) => bail!(
                "Guild {} | Error joining the channel: {}",
                self.guild_id,
                error
            ),
        };

        let handle = handle.lock().await;

        handle.add_global_event(
            Event::Track(TrackEvent::End),
            SongbirdEventHandler::new(self.guild_id, self.songbird.clone()),
        );

        Ok(())
    }

    async fn assure_connected(&self, ctx: Context<'_>) -> Result<(), Error> {
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
                        current_connection.channel_id.map(|c| c.0) != Some(channel.get())
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

    pub async fn stop_player(&self) -> Result<(), Error> {
        self.songbird.remove(self.guild_id).await?;

        let nodes = self.lava_client.nodes().await;
        nodes.remove(&self.guild_id);

        let loops = self.lava_client.loops().await;
        loops.remove(&self.guild_id);

        self.lava_client.destroy(self.guild_id).await?;

        Ok(())
    }

    async fn queue_music(&self, ctx: Context<'_>, query: String) -> Result<(), Error> {
        let search_results = YoutubeDl::new_search(self.http_client, query)
            .search(1)
            .await?;

        match search_results.first() {
            Some(track) => self.add_to_queue(ctx, track).await,
            None => {
                let reply = "Could not find any video of the search query.";

                ctx.say(reply).await?;

                Ok(())
            }
        }
    }

    async fn queue_playlist(&self, ctx: Context<'_>, query: String) -> Result<(), Error> {
        let query_information = self.lava_client.auto_search_tracks(&query).await?;

        for track in query_information.tracks.iter() {
            self.add_track_to_queue(track).await?;
        }

        let reply = format!(
            "Added {} tracks to the queue",
            query_information.tracks.len()
        );

        ctx.say(reply).await?;

        Ok(())
    }

    async fn add_to_queue(&self, ctx: Context<'_>, track: &AuxMetadata) -> Result<(), Error> {
        self.add_track_to_queue(track).await?;

        ctx.say(format!("Added to queue: {}", get_track_name(track)))
            .await?;

        Ok(())
    }

    async fn add_track_to_queue(&self, track: &AuxMetadata) -> Result<(), Error> {
        let handle = self
            .songbird
            .get(self.guild_id)
            .ok_or_else(anyhow!("Error obtaining songbird guild call"))?;

        let handle = handle.lock().await;

        handle.play_input(track);

        self.insert_jukebox_use(track);

        Ok(())
    }

    async fn skip_track(&self) -> Result<Option<TrackQueue>, Error> {
        let skipped_track = self.lava_client.skip(self.guild_id).await;

        if skipped_track.is_some() {
            let nodes = self.lava_client.nodes().await;

            let node = nodes
                .get(&self.guild_id)
                .ok_or_else(|| anyhow!("[Skip] Couldn't get node for {}", self.guild_id))?;

            if node.queue.is_empty() {
                self.lava_client.stop(self.guild_id).await?;
            }
        }

        Ok(skipped_track)
    }

    async fn shuffle_playlist(&self) -> Result<(), Error> {
        let nodes = self.lava_client.nodes().await;

        let mut node = nodes
            .get_mut(&self.guild_id)
            .ok_or_else(|| anyhow!("[Shuffle] Couldn't get node for {}", self.guild_id))?;

        let now_playing = node.queue.remove(0);

        node.queue.shuffle(rand::thread_rng().borrow_mut());

        node.queue.insert(0, now_playing);

        Ok(())
    }

    fn insert_jukebox_use(&self, track: &Track) {
        let service = self.jukebox_repository.to_owned();

        let j_use = JukeboxUse::new(self.guild_id, self.user_id, track);

        tokio::spawn(async move { service.add_jukebox_use(j_use).await.log() });
    }
}

fn get_track_name(track: &Track) -> &str {
    track
        .info
        .as_ref()
        .map(|i| i.title.as_str())
        .unwrap_or_else(|| track.track.as_str())
}
