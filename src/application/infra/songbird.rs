use std::sync::Arc;

use anyhow::{anyhow, Error};
use lavalink_rs::model::Track;
use songbird::Songbird;

use crate::extensions::serenity::{context_ext::ContextExt, serenity_structs::Context};

pub struct ContextSongbird {
    guild_id: u64,
    songbird: Arc<Songbird>,
}

impl ContextSongbird {
    pub fn new(guild_id: u64, songbird: Arc<Songbird>) -> Self {
        Self { guild_id, songbird }
    }

    pub async fn queue(&self, ctx: Context<'_>, query: String) -> Result<(), Error> {
        match self.songbird.get(self.guild_id) {
            Some(_) => self.queue_music(ctx, query).await,
            None => {
                self.join_voice_channel(ctx).await?;
                self.queue_music(ctx, query).await
            }
        }
    }

    async fn queue_music(&self, ctx: Context<'_>, query: String) -> Result<(), Error> {
        let lava_client = &ctx.data().lava_client;

        let query_information = lava_client.auto_search_tracks(&query).await?;

        match query_information.tracks.first() {
            Some(t) => self.add_to_queue(ctx, t.to_owned()).await,
            None => {
                let reply = "Could not find any video of the search query.";

                ctx.say(reply).await?;

                Ok(())
            }
        }
    }

    async fn add_to_queue(&self, ctx: Context<'_>, track: Track) -> Result<(), Error> {
        let message = {
            let track = track.to_owned();
            let msg = track
                .info
                .map(|i| i.title.to_owned())
                .unwrap_or_else(|| track.track);

            format!("Added to queue: {msg}")
        };

        let lava_client = &ctx.data().lava_client;

        lava_client.play(self.guild_id, track).queue().await?;

        ctx.say(message).await?;

        Ok(())
    }

    async fn join_voice_channel(&self, ctx: Context<'_>) -> Result<(), Error> {
        let guild = ctx.assure_cached_guild()?;

        let channel_id = guild
            .voice_states
            .get(&ctx.author().id)
            .and_then(|voice_state| voice_state.channel_id);

        let connect_to = match channel_id {
            Some(channel) => channel,
            None => {
                ctx.say("Join a voice channel.").await?;

                return Ok(());
            }
        };

        let (_, handler) = self.songbird.join_gateway(self.guild_id, connect_to).await;

        match handler {
            Ok(_connection_info) => {
                let _lava_client = &ctx.data().lava_client;
                //lava_client.create_session(&connection_info).await?;
                Ok(())
            }
            Err(error) => {
                let msg = format!(
                    "Guild {} | Error joining the channel: {}",
                    self.guild_id, error
                );

                ctx.say(msg).await?;

                Err(anyhow!(error))
            }
        }
    }
}
