use std::sync::Arc;

use anyhow::{anyhow, bail, Result};
use futures::StreamExt;
use lavalink_rs::{
    client::LavalinkClient,
    hook,
    model::events::TrackStart,
    player_context::{PlayerContext, TrackInQueue},
    prelude::{SearchEngines, TrackLoadData},
};
use poise::serenity_prelude::{ChannelId, GuildId, Http, MessageBuilder, UserId};
use songbird::Songbird;

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
        ctx.say("Shuffled queue!").await?;

        bail!("todo!")
    }

    pub async fn stop(&self, ctx: Context<'_>) -> Result<()> {
        self.stop_player().await?;

        ctx.say("Player stopped! Queue cleared!").await?;

        Ok(())
    }

    pub async fn skip(&self, _ctx: Context<'_>) -> Result<()> {
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

                    let lines: Vec<_> = queue
                        .enumerate()
                        .take(MAX_QUEUE_DESCRIPTION_SIZE)
                        .map(|(i, track)| {
                            let now = if i == 0 { "▶️" } else { "" };

                            format!(
                                "- {} {:?} | By: <@{}>",
                                now, track.track, track.track.info.title
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

    pub async fn join_voice_channel(&self, channel_id: ChannelId) -> Result<()> {
        let (connection_info, _) = self
            .songbird
            .join_gateway(self.guild_id, channel_id)
            .await?;

        self.lavalink
            .create_player_context(self.guild_id, connection_info)
            .await
            .map_err(|e| anyhow!("Guild {} | Error joining the channel: {}", self.guild_id, e))?;

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

        // let should_join = match self.songbird.get(self.guild_id) {
        //     Some(call) => {
        //         let guard = call.lock().await;
        //
        //         match guard.current_connection() {
        //             Some(current_connection) => {
        //                 current_connection.channel_id.map(|c| c.0.get()) != Some(channel.get())
        //             }
        //             None => true,
        //         }
        //     }
        //     None => true,
        // };

        // if true {
        self.join_voice_channel(channel).await?;
        // }

        Ok(())
    }

    pub async fn stop_player(&self) -> Result<()> {
        bail!("todo!")
    }

    async fn queue_music(&self, ctx: Context<'_>, query: String) -> Result<()> {
        let player = self.get_player_ctx()?;

        let query = SearchEngines::YouTube.to_query(&query)?;

        let loaded_tracks = self.lavalink.load_tracks(self.guild_id, &query).await?;

        let track_data = loaded_tracks
            .data
            .ok_or_else(|| anyhow!("Failed to get data about this track"))?;

        let tracks: Vec<TrackInQueue> = match &track_data {
            TrackLoadData::Track(t) => vec![t.clone().into()],
            TrackLoadData::Search(s) => vec![s[0].clone().into()],
            TrackLoadData::Playlist(p) => p.tracks.iter().map(|x| x.clone().into()).collect(),
            TrackLoadData::Error(e) => bail!("Error getting track data | {:?}", e),
        };

        let msg = match track_data {
            TrackLoadData::Playlist(p) => format!("Added {} tracks to the queue", p.tracks.len()),
            _ => {
                let track = &tracks[0].track;
                format!(
                    "Added {} - {} to the queue",
                    track.info.title, track.info.author
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

        let queue = player.get_queue();
        queue.append(tracks.into())?;

        self.jukebox_repository.add_jukebox_use(jukebox_use).await?;

        ctx.say(msg).await?;

        Ok(())
    }

    fn get_player_ctx(&self) -> Result<PlayerContext> {
        self.lavalink
            .get_player_context(self.guild_id)
            .ok_or_else(|| anyhow!("Error getting player context"))
    }
}

struct PlayerContextData {
    channel: ChannelId,
    http: Arc<Http>,
}

#[hook]
pub async fn track_start(client: LavalinkClient, _session_id: String, event: &TrackStart) {
    track_start_handler(client, event).await.log();
}

async fn track_start_handler(client: LavalinkClient, event: &TrackStart) -> Result<()> {
    let player = client
        .get_player_context(event.guild_id)
        .ok_or_else(|| anyhow!("Couldn't get player context"))?;

    let data = player.data::<(ChannelId, std::sync::Arc<Http>)>()?;

    let (channel_id, http) = (&data.0, &data.1);

    let msg = {
        let track = &event.track;

        if let Some(uri) = &track.info.uri {
            format!(
                "Now playing: [{} - {}](<{}>) | Requested by <@!{}>",
                track.info.author,
                track.info.title,
                uri,
                track.user_data.clone().unwrap()["requester_id"]
            )
        } else {
            format!(
                "Now playing: {} - {} | Requested by <@!{}>",
                track.info.author,
                track.info.title,
                track.user_data.clone().unwrap()["requester_id"]
            )
        }
    };

    channel_id.say(http, msg).await?;

    Ok(())
}
