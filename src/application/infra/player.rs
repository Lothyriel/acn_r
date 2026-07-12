use anyhow::{Result, anyhow, bail};
use futures::StreamExt;
use lavalink_rs::{
    client::LavalinkClient,
    hook,
    model::events::{Stats, TrackEnd, TrackStart},
    model::player::ConnectionInfo as LavalinkConnectionInfo,
    model::track::TrackLoadType,
    player_context::{PlayerContext, TrackInQueue},
    prelude::{SearchEngines, TrackLoadData},
};
use poise::serenity_prelude::{ChannelId, GuildId, Http, MessageBuilder, UserId};
use rand::seq::SliceRandom;
use serde_json::json;
use songbird::Songbird;
use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};
use tokio::sync::{Mutex, watch};

use crate::{
    application::{
        models::entities::jukebox_use::{JukeboxUse, TrackMetadata},
        repositories::jukebox::JukeboxRepository,
    },
    extensions::{
        log_ext::LogExt,
        serenity::{Context, context_ext::ContextExt},
    },
};

pub struct AudioPlayer {
    guild_id: GuildId,
    user_id: UserId,
    jukebox_repository: JukeboxRepository,
    songbird: Arc<Songbird>,
    lavalink: LavalinkClient,
}

const IDLE_DISCONNECT_TIMEOUT: Duration = Duration::from_secs(60);

fn is_idle(active_track: bool, queue_count: usize) -> bool {
    !active_track && queue_count == 0
}

pub struct LavalinkRuntime {
    pub songbird: Arc<Songbird>,
    idle_disconnect: IdleDisconnect,
}

impl LavalinkRuntime {
    pub fn new(songbird: Arc<Songbird>) -> Self {
        Self {
            songbird,
            idle_disconnect: IdleDisconnect::default(),
        }
    }
}

#[derive(Default)]
struct IdleDisconnect {
    next_generation: AtomicU64,
    timers: Mutex<HashMap<GuildId, IdleTimer>>,
}

struct IdleTimer {
    generation: u64,
    cancel: watch::Sender<()>,
}

impl IdleDisconnect {
    async fn schedule(&self, guild_id: GuildId) -> (u64, watch::Receiver<()>) {
        let generation = self.next_generation.fetch_add(1, Ordering::Relaxed);
        let (cancel, receiver) = watch::channel(());
        let previous = self
            .timers
            .lock()
            .await
            .insert(guild_id, IdleTimer { generation, cancel });

        if let Some(previous) = previous {
            let _ = previous.cancel.send(());
        }

        (generation, receiver)
    }

    async fn cancel(&self, guild_id: GuildId) {
        if let Some(timer) = self.timers.lock().await.remove(&guild_id) {
            let _ = timer.cancel.send(());
        }
    }

    async fn is_current(&self, guild_id: GuildId, generation: u64) -> bool {
        self.timers
            .lock()
            .await
            .get(&guild_id)
            .is_some_and(|timer| timer.generation == generation)
    }

    async fn clear_if_current(&self, guild_id: GuildId, generation: u64) {
        let mut timers = self.timers.lock().await;
        if timers
            .get(&guild_id)
            .is_some_and(|timer| timer.generation == generation)
        {
            timers.remove(&guild_id);
        }
    }
}

impl AudioPlayer {
    pub fn new(
        guild_id: GuildId,
        user_id: UserId,
        jukebox_repository: JukeboxRepository,
        songbird: Arc<Songbird>,
        lavalink: LavalinkClient,
    ) -> Self {
        Self {
            guild_id,
            user_id,
            jukebox_repository,
            songbird,
            lavalink,
        }
    }

    pub async fn shuffle(&self, ctx: Context<'_>) -> Result<()> {
        let player_ctx = self.get_player_ctx()?;

        let queue_ref = player_ctx.get_queue();

        let mut queue = queue_ref.get_queue().await?;

        queue.make_contiguous().shuffle(&mut rand::rng());

        queue_ref.replace(queue)?;

        ctx.say("Queue shuffled!").await?;

        Ok(())
    }

    pub async fn stop(&self, ctx: Context<'_>) -> Result<()> {
        let msg = match self.stop_player().await? {
            true => "Player stopped! Queue cleared!",
            false => "Nothing to clear",
        };

        ctx.say(msg).await?;

        Ok(())
    }

    pub async fn skip(&self, ctx: Context<'_>) -> Result<()> {
        let player = self.get_player_ctx()?;

        let now_playing = player.get_player().await?.track;

        if let Some(np) = now_playing {
            player.skip()?;
            ctx.say(format!("Skipped {}", np.info.title)).await?;
        } else {
            ctx.say("Nothing to skip").await?;
        }

        Ok(())
    }

    pub async fn show_queue(&self, ctx: Context<'_>) -> Result<()> {
        const MAX_QUEUE_DESCRIPTION_SIZE: usize = 10;

        let queue_description = {
            let player = self.get_player_ctx()?;
            let now_playing = player.get_player().await?.track;

            let queue = player.get_queue();

            let count = queue.get_count().await?;

            let mut message_builder = MessageBuilder::new();

            match (now_playing, count == 0) {
                (Some(track), _) => {
                    let info = track.info;
                    let uri = info
                        .uri
                        .as_deref()
                        .unwrap_or("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
                    let total_seconds = info.length / 1000;
                    let minutes = total_seconds / 60;
                    let seconds = total_seconds % 60;

                    message_builder.push_line("Now playing:");
                    message_builder.push_line(format!(
                        "{:02}:{:02} - [{}]({})",
                        minutes, seconds, info.title, uri,
                    ));

                    if count > 0 {
                        message_builder.push_line("");
                        message_builder.push_line("Queue:");
                        message_builder.push_line("");

                        let lines: Vec<_> =
                            queue
                                .take(MAX_QUEUE_DESCRIPTION_SIZE)
                                .map(|track| {
                                    let info = track.track.info;

                                    let uri = info
                                        .uri
                                        .as_deref()
                                        .unwrap_or("https://www.youtube.com/watch?v=dQw4w9WgXcQ");

                                    let total_seconds = info.length / 1000;
                                    let minutes = total_seconds / 60;
                                    let seconds = total_seconds % 60;

                                    format!(
                                        "{:02}:{:02} - [{}]({})",
                                        minutes, seconds, info.title, uri,
                                    )
                                })
                                .collect()
                                .await;

                        for line in lines {
                            message_builder.push_line(line);
                        }

                        if count > MAX_QUEUE_DESCRIPTION_SIZE {
                            message_builder.push(format!("{} more tracks...", count - 10));
                        }
                    }
                }
                (None, false) => {
                    message_builder.push_line("Queue: ");
                    message_builder.push_line("");

                    let lines: Vec<_> = queue
                        .take(MAX_QUEUE_DESCRIPTION_SIZE)
                        .map(|track| {
                            let info = track.track.info;

                            let uri = info
                                .uri
                                .as_deref()
                                .unwrap_or("https://www.youtube.com/watch?v=dQw4w9WgXcQ");

                            let total_seconds = info.length / 1000;
                            let minutes = total_seconds / 60;
                            let seconds = total_seconds % 60;

                            format!("{:02}:{:02} - [{}]({})", minutes, seconds, info.title, uri,)
                        })
                        .collect()
                        .await;

                    for line in lines {
                        message_builder.push_line(line);
                    }

                    if count > MAX_QUEUE_DESCRIPTION_SIZE {
                        message_builder.push(format!("{} more tracks...", count - 10));
                    }
                }
                (None, true) => {
                    message_builder.push_line("EMPTY!!!");
                }
            };

            message_builder.build()
        };

        ctx.say(queue_description).await?;

        Ok(())
    }

    pub async fn play(&self, ctx: Context<'_>, query: String) -> Result<()> {
        match self.assure_connected(ctx).await? {
            true => self.queue_music(ctx, query).await,
            false => Ok(()),
        }
    }

    pub async fn join_voice_channel(&self, channel_id: ChannelId, http: Arc<Http>) -> Result<()> {
        let (connection_info, _) = self
            .songbird
            .join_gateway(self.guild_id, channel_id)
            .await?;

        let mut connection_info = LavalinkConnectionInfo {
            endpoint: connection_info.endpoint,
            token: connection_info.token,
            session_id: connection_info.session_id,
            channel_id: Some(channel_id.into()),
        };

        connection_info.fix();

        self.lavalink
            .create_player_context_with_data::<(ChannelId, std::sync::Arc<Http>)>(
                self.guild_id,
                connection_info,
                std::sync::Arc::new((channel_id, http)),
            )
            .await
            .map_err(|e| anyhow!("Guild {} | Error joining the channel: {}", self.guild_id, e))?;

        Ok(())
    }

    async fn assure_connected(&self, ctx: Context<'_>) -> Result<bool> {
        let channel = match ctx.get_author_voice_channel().await? {
            Some(c) => c,
            None => {
                ctx.say("Please join a voice channel.").await?;
                return Ok(false);
            }
        };

        let should_join = match self.songbird.get(self.guild_id) {
            Some(call) => {
                let guard = call.lock().await;

                match guard.current_connection() {
                    Some(current_connection) => {
                        current_connection.channel_id.0.get() != channel.get()
                    }
                    None => true,
                }
            }
            None => true,
        };

        if should_join {
            self.join_voice_channel(channel, ctx.serenity_context().http.clone())
                .await?;
        }

        Ok(true)
    }

    pub async fn stop_player(&self) -> Result<bool> {
        let player_ctx = self.get_player_ctx()?;

        let now_playing = player_ctx.get_player().await?.track;

        let result = if now_playing.is_some() {
            self.cancel_idle_disconnect().await?;
            player_ctx.stop_now().await?;
            let queue = player_ctx.get_queue();
            queue.clear()?;
            self.lavalink.delete_player(self.guild_id).await?;

            self.songbird.remove(self.guild_id).await?;
            true
        } else {
            false
        };

        Ok(result)
    }

    async fn queue_music(&self, ctx: Context<'_>, query: String) -> Result<()> {
        let player_ctx = self.get_player_ctx()?;

        let original_query = query;
        let query = if is_url(&original_query) {
            original_query.clone()
        } else {
            SearchEngines::YouTube.to_query(&original_query)?
        };

        let loaded_tracks = self.lavalink.load_tracks(self.guild_id, &query).await?;

        let mut tracks: Vec<TrackInQueue> = match loaded_tracks.load_type {
            TrackLoadType::Track => match loaded_tracks.data {
                Some(TrackLoadData::Track(track)) => vec![track.into()],
                _ => bail!("Lavalink returned an invalid track response"),
            },
            TrackLoadType::Search => match loaded_tracks.data {
                Some(TrackLoadData::Search(search_results)) => {
                    let first_track = match search_results.into_iter().next() {
                        Some(track) => track,
                        None => {
                            ctx.say(format!("No search results found for `{}`.", original_query))
                                .await?;
                            return Ok(());
                        }
                    };

                    vec![first_track.into()]
                }
                _ => bail!("Lavalink returned an invalid search response"),
            },
            TrackLoadType::Playlist => match loaded_tracks.data {
                Some(TrackLoadData::Playlist(playlist)) => {
                    if playlist.tracks.is_empty() {
                        ctx.say(format!("The playlist for `{}` is empty.", original_query))
                            .await?;
                        return Ok(());
                    }

                    playlist.tracks.into_iter().map(Into::into).collect()
                }
                _ => bail!("Lavalink returned an invalid playlist response"),
            },
            TrackLoadType::Empty => {
                ctx.say(format!("No tracks found for `{}`.", original_query))
                    .await?;
                return Ok(());
            }
            TrackLoadType::Error => match loaded_tracks.data {
                Some(TrackLoadData::Error(error)) => {
                    bail!("Couldn't load that track: {}", error.message)
                }
                _ => bail!("Lavalink returned an unknown track loading error"),
            },
        };

        self.cancel_idle_disconnect().await?;

        let requester_id = ctx.author().id.get();

        for track in &mut tracks {
            track.track.user_data = Some(json!({ "requester_id": requester_id }));
        }

        let track_count = tracks.len();

        let msg = match track_count {
            count if count > 1 => format!("Added {} tracks to the queue", count),
            _ => {
                let track = &tracks[0].track;
                format!(
                    "{} Added [{}]({}) to the queue",
                    ctx.author(),
                    track.info.title,
                    track
                        .info
                        .uri
                        .as_deref()
                        .unwrap_or("https://www.youtube.com/watch?v=dQw4w9WgXcQ")
                )
            }
        };

        let track = tracks[0].track.clone();

        let jukebox_use = JukeboxUse {
            time: chrono::Utc::now(),
            guild_id: self.guild_id.get(),
            user_id: self.user_id.get(),
            info: TrackMetadata {
                author: track.info.author.clone(),
                title: track.info.title.clone(),
                uri: track.info.uri.clone(),
                seconds: track.info.length.checked_div(1000).unwrap_or(0),
            },
        };

        let has_active_track = player_ctx.get_player().await?.track.is_some();
        let queue = player_ctx.get_queue();

        if has_active_track {
            queue.append(tracks.into())?;
        } else {
            let mut tracks = tracks.into_iter();

            let track_to_play = tracks
                .next()
                .ok_or_else(|| anyhow!("No tracks available to play"))?;

            if !tracks.as_slice().is_empty() {
                queue.append(tracks.collect::<Vec<_>>().into())?;
            }

            player_ctx.play(&track_to_play.track).await?;
        }

        self.jukebox_repository.add_jukebox_use(jukebox_use).await?;

        ctx.say(msg).await?;

        Ok(())
    }

    async fn cancel_idle_disconnect(&self) -> Result<()> {
        let runtime = self.lavalink.data::<LavalinkRuntime>()?;
        runtime.idle_disconnect.cancel(self.guild_id).await;

        Ok(())
    }

    fn get_player_ctx(&self) -> Result<PlayerContext> {
        self.lavalink
            .get_player_context(self.guild_id)
            .ok_or_else(|| anyhow!("Error getting player context"))
    }
}

fn is_url(query: &str) -> bool {
    let parsed = url::Url::parse(query);
    matches!(
        parsed.as_ref().ok().map(|u| u.scheme()),
        Some("http" | "https")
    )
}

#[hook]
pub async fn track_start(client: LavalinkClient, _session_id: String, event: &TrackStart) {
    track_start_handler(client, event).await.log();
}

#[hook]
pub async fn track_end(client: LavalinkClient, _session_id: String, event: &TrackEnd) {
    track_end_handler(client, event).await.log();
}

#[hook]
pub async fn stats(_client: LavalinkClient, _session_id: String, event: &Stats) {
    log::debug!("{:?}", event);
}

async fn track_start_handler(client: LavalinkClient, event: &TrackStart) -> Result<()> {
    let runtime = client.data::<LavalinkRuntime>()?;
    runtime
        .idle_disconnect
        .cancel(GuildId::new(event.guild_id.0))
        .await;

    let msg = {
        let track = &event.track;

        let requester = track
            .user_data
            .as_ref()
            .and_then(|data| data.get("requester_id"))
            .and_then(|value| value.as_u64())
            .map(|id| format!(" | Requested by <@!{}>", id))
            .unwrap_or_default();

        if let Some(uri) = &track.info.uri {
            format!(
                "Now playing: [{} - {}](<{}>){}",
                track.info.author, track.info.title, uri, requester
            )
        } else {
            format!(
                "Now playing: {} - {}{}",
                track.info.author, track.info.title, requester
            )
        }
    };

    log::info!("{msg}");

    Ok(())
}

async fn track_end_handler(client: LavalinkClient, event: &TrackEnd) -> Result<()> {
    let runtime = client.data::<LavalinkRuntime>()?;
    let guild_id = GuildId::new(event.guild_id.0);
    let lavalink_guild_id = event.guild_id;
    let (generation, mut cancellation) = runtime.idle_disconnect.schedule(guild_id).await;

    tokio::spawn(async move {
        tokio::select! {
            _ = tokio::time::sleep(IDLE_DISCONNECT_TIMEOUT) => {}
            _ = cancellation.changed() => return,
        }

        if !runtime
            .idle_disconnect
            .is_current(guild_id, generation)
            .await
        {
            return;
        }

        let Some(player_ctx) = client.get_player_context(lavalink_guild_id) else {
            runtime
                .idle_disconnect
                .clear_if_current(guild_id, generation)
                .await;
            return;
        };

        let result: Result<bool> = async {
            let player = player_ctx.get_player().await?;
            let queue_count = player_ctx.get_queue().get_count().await?;

            if !is_idle(player.track.is_some(), queue_count) {
                return Ok(false);
            }

            if !runtime
                .idle_disconnect
                .is_current(guild_id, generation)
                .await
            {
                return Ok(false);
            }

            player_ctx.get_queue().clear()?;
            client.delete_player(lavalink_guild_id).await?;
            runtime.songbird.remove(guild_id).await?;

            Ok(true)
        }
        .await;

        runtime
            .idle_disconnect
            .clear_if_current(guild_id, generation)
            .await;

        match result {
            Ok(true) => log::info!("Guild {} | Disconnected after idle playback", guild_id),
            Ok(false) => {}
            Err(error) => log::error!("Guild {} | Idle disconnect failed: {}", guild_id, error),
        }
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn idle_requires_no_active_track_and_no_queued_tracks() {
        assert!(is_idle(false, 0));
        assert!(!is_idle(true, 0));
        assert!(!is_idle(false, 1));
    }

    #[tokio::test]
    async fn scheduling_a_new_timer_cancels_the_previous_timer() {
        let idle_disconnect = IdleDisconnect::default();
        let guild_id = GuildId::new(1);

        let (old_generation, mut old_cancellation) = idle_disconnect.schedule(guild_id).await;
        let (new_generation, _new_cancellation) = idle_disconnect.schedule(guild_id).await;

        assert!(!idle_disconnect.is_current(guild_id, old_generation).await);
        assert!(idle_disconnect.is_current(guild_id, new_generation).await);
        assert!(old_cancellation.changed().await.is_ok());
    }

    #[tokio::test]
    async fn cancelling_a_timer_invalidates_its_generation() {
        let idle_disconnect = IdleDisconnect::default();
        let guild_id = GuildId::new(1);

        let (generation, mut cancellation) = idle_disconnect.schedule(guild_id).await;
        idle_disconnect.cancel(guild_id).await;

        assert!(!idle_disconnect.is_current(guild_id, generation).await);
        assert!(cancellation.changed().await.is_ok());
    }
}
