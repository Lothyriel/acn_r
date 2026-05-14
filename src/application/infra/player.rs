use anyhow::{anyhow, bail, Result};
use futures::StreamExt;
use lavalink_rs::{
    client::LavalinkClient,
    hook,
    model::events::{Stats, TrackStart},
    model::player::ConnectionInfo as LavalinkConnectionInfo,
    model::track::TrackLoadType,
    player_context::{PlayerContext, TrackInQueue},
    prelude::{SearchEngines, TrackLoadData},
};
use poise::serenity_prelude::{ChannelId, GuildId, Http, MessageBuilder, UserId};
use rand::seq::SliceRandom;
use serde_json::json;
use songbird::Songbird;
use std::sync::Arc;

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

pub struct AudioPlayer {
    guild_id: GuildId,
    user_id: UserId,
    jukebox_repository: JukeboxRepository,
    songbird: Arc<Songbird>,
    lavalink: LavalinkClient,
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

            let queue = player.get_queue();

            let count = queue.get_count().await?;

            let mut message_builder = MessageBuilder::new();

            match count == 0 {
                false => {
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
        let query = SearchEngines::YouTube.to_query(&original_query)?;

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

        let requester_id = ctx.author().id.get();

        for track in &mut tracks {
            track.track.user_data = Some(json!({ "requester_id": requester_id }));
        }

        let msg = match tracks.len() {
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

        let queue = player_ctx.get_queue();
        queue.append(tracks.into())?;

        self.jukebox_repository.add_jukebox_use(jukebox_use).await?;

        player_ctx.play(&track).await?;

        ctx.say(msg).await?;

        Ok(())
    }

    fn get_player_ctx(&self) -> Result<PlayerContext> {
        self.lavalink
            .get_player_context(self.guild_id)
            .ok_or_else(|| anyhow!("Error getting player context"))
    }
}

#[hook]
pub async fn track_start(client: LavalinkClient, _session_id: String, event: &TrackStart) {
    track_start_handler(client, event).await.log();
}

#[hook]
pub async fn stats(_client: LavalinkClient, _session_id: String, event: &Stats) {
    log::warn!("{:?}", event);
}

async fn track_start_handler(client: LavalinkClient, event: &TrackStart) -> Result<()> {
    let player = client
        .get_player_context(event.guild_id)
        .ok_or_else(|| anyhow!("Couldn't get player context"))?;

    let data = player.data::<(ChannelId, std::sync::Arc<Http>)>()?;

    let (channel_id, http) = (&data.0, &data.1);

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
                track.info.author,
                track.info.title,
                uri,
                requester
            )
        } else {
            format!(
                "Now playing: {} - {}{}",
                track.info.author,
                track.info.title,
                requester
            )
        }
    };

    channel_id.say(http, msg).await?;

    Ok(())
}
