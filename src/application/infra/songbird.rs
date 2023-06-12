use anyhow::{anyhow, Error};
use lavalink_rs::{async_trait, gateway::LavalinkEventHandler, model::Track, LavalinkClient};
use poise::serenity_prelude::Http;
use songbird::Songbird;
use std::sync::Arc;

use crate::{
    application::{
        models::entities::jukebox_use::JukeboxUse, services::jukebox_services::JukeboxServices,
    },
    extensions::{
        log_ext::LogExt,
        serenity::{context_ext::ContextExt, serenity_structs::Context},
    },
    infra::{appsettings::AppSettings, env},
};

struct LavalinkHandler;

#[async_trait]
impl LavalinkEventHandler for LavalinkHandler {}

pub async fn get_lavalink_client(
    token: &str,
    settings: &AppSettings,
) -> Result<LavalinkClient, Error> {
    let app_info = Http::new(token).get_current_application_info().await?;

    let lava_client = LavalinkClient::builder(app_info.id.0)
        .set_host(&settings.lavalink_url)
        .set_password(env::get("LAVALINK_PASSWORD")?)
        .build(LavalinkHandler)
        .await?;

    Ok(lava_client)
}

pub struct ContextSongbird {
    guild_id: u64,
    user_id: u64,
    songbird: Arc<Songbird>,
    lava_client: LavalinkClient,
    jukebox_services: JukeboxServices,
}

impl ContextSongbird {
    pub fn new(
        guild_id: u64,
        user_id: u64,
        songbird: Arc<Songbird>,
        lava_client: LavalinkClient,
        jukebox_services: JukeboxServices,
    ) -> Self {
        Self {
            guild_id,
            user_id,
            songbird,
            lava_client,
            jukebox_services,
        }
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
        let query_information = self.lava_client.auto_search_tracks(&query).await?;

        match query_information.tracks.first() {
            Some(track) => self.add_to_queue(ctx, track.to_owned()).await,
            None => {
                let reply = "Could not find any video of the search query.";

                ctx.say(reply).await?;

                Ok(())
            }
        }
    }

    async fn add_to_queue(&self, ctx: Context<'_>, track: Track) -> Result<(), Error> {
        self.lava_client
            .play(self.guild_id, track.to_owned())
            .queue()
            .await?;

        self.add_jukebox_use(&track);

        ctx.say(format!("Added to queue: {}", get_track_name(&track)))
            .await?;

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
                self.lava_client
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

    fn add_jukebox_use(&self, track: &Track) {
        let service = self.jukebox_services.to_owned();

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
