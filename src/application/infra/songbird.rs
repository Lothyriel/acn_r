use std::sync::Arc;

use anyhow::{anyhow, Error};
use lavalink_rs::{async_trait, gateway::LavalinkEventHandler, model::Track, LavalinkClient};
use poise::serenity_prelude::Http;
use songbird::Songbird;

use crate::extensions::serenity::{context_ext::ContextExt, serenity_structs::Context};
use crate::infra::{appsettings::AppSettings, env};

struct LavalinkHandler;

#[async_trait]
impl LavalinkEventHandler for LavalinkHandler {}

pub async fn get_lavalink_client(
    token: &str,
    settings: &AppSettings,
) -> Result<LavalinkClient, Error> {
    let bot_id = Http::new(token).get_current_application_info().await?;

    let lava_client = LavalinkClient::builder(bot_id.id.0)
        .set_host(&settings.lavalink_url)
        .set_password(env::get("LAVALINK_PASSWORD")?)
        .build(LavalinkHandler)
        .await?;

    Ok(lava_client)
}

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
        let message = get_track_name(&track);

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
            Ok(connection_info) => {
                let lava_client = &ctx.data().lava_client;
                lava_client
                    .create_session_with_songbird(&connection_info)
                    .await?;
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

fn get_track_name(track: &Track) -> String {
    let message = {
        let msg = track
            .info
            .as_ref()
            .map(|i| i.title.as_str())
            .unwrap_or_else(|| track.track.as_str());

        format!("Added to queue: {msg}")
    };
    message
}
