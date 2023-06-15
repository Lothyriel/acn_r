use std::sync::Arc;

use anyhow::{anyhow, Error};
use lavalink_rs::{async_trait, gateway::LavalinkEventHandler, model::Track, LavalinkClient};
use poise::serenity_prelude::{ChannelId, Http, Mentionable, MessageBuilder};
use songbird::Songbird;

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
        .set_host(&settings.lavalink_settings.url)
        .set_port(settings.lavalink_settings.port)
        .set_password(env::get("LAVALINK_PASSWORD")?)
        .build(LavalinkHandler)
        .await?;

    Ok(lava_client)
}

pub struct SongbirdCtx {
    guild_id: u64,
    user_id: u64,
    songbird: Arc<Songbird>,
    lava_client: LavalinkClient,
    jukebox_services: JukeboxServices,
}

impl SongbirdCtx {
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

    pub async fn skip(&self, ctx: Context<'_>) -> Result<(), Error> {
        let message = match self.lava_client.skip(self.guild_id).await {
            Some(track) => {
                let nodes = self.lava_client.nodes().await;

                let node = nodes
                    .get(&self.guild_id)
                    .ok_or_else(|| anyhow!("Couldn't get node for {}", self.guild_id))?;

                if node.queue.is_empty() {
                    self.lava_client.stop(self.guild_id).await?;
                }

                format!(
                    "{} Skipped: {}",
                    ctx.author().mention(),
                    get_track_name(&track.track)
                )
            }
            None => "Nothing to skip.".to_owned(),
        };

        ctx.say(message).await?;

        Ok(())
    }

    pub async fn show_queue(&self, ctx: Context<'_>) -> Result<(), Error> {
        let queue = {
            let nodes = self.lava_client.nodes().await;

            let node = nodes
                .get(&self.guild_id)
                .ok_or_else(|| anyhow!("Couldn't get node for {}", self.guild_id))?;

            let mut message_builder = MessageBuilder::new();

            match node.queue.is_empty() {
                false => {
                    message_builder.push_line("Queue: ");

                    for track in node.queue.iter() {
                        let track_name = get_track_name(&track.track);

                        let requester = track
                            .requester
                            .map(|r| format!("<@{}>", r.0))
                            .unwrap_or_else(|| "Unknown".to_owned());

                        let line = format!("- {}  --- By: {}", track_name, requester);

                        message_builder.push_line(line);
                    }
                }
                true => {
                    message_builder.push_line("EMPTY!!!");
                }
            };

            message_builder.build()
        };

        ctx.say(queue).await?;

        Ok(())
    }

    pub async fn play(&self, ctx: Context<'_>, query: String) -> Result<(), Error> {
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
            .requester(self.user_id)
            .queue()
            .await?;

        self.add_jukebox_use(&track);

        ctx.say(format!("Added to queue: {}", get_track_name(&track)))
            .await?;

        Ok(())
    }

    async fn join_voice_channel(&self, ctx: Context<'_>) -> Result<(), Error> {
        let connect_to = get_author_voice_channel(ctx).await?;

        let (_, handler) = self.songbird.join_gateway(self.guild_id, connect_to).await;

        match handler {
            Ok(connection_info) => {
                self.lava_client
                    .create_session_with_songbird(&connection_info)
                    .await?;
                Ok(())
            }
            Err(error) => {
                let msg = format!("Error joining the channel: {}", error);

                ctx.say(&msg).await?;

                Err(anyhow!("Guild {} | {}", self.guild_id, msg))
            }
        }
    }

    fn add_jukebox_use(&self, track: &Track) {
        let service = self.jukebox_services.to_owned();

        let j_use = JukeboxUse::new(self.guild_id, self.user_id, track);

        tokio::spawn(async move { service.add_jukebox_use(j_use).await.log() });
    }
}

async fn get_author_voice_channel(ctx: Context<'_>) -> Result<ChannelId, Error> {
    let guild = ctx.assure_cached_guild()?;
    let channel_id = guild
        .voice_states
        .get(&ctx.author().id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            ctx.say("Join a voice channel.").await?;

            return Err(anyhow!("User is not connected to a voice channel"));
        }
    };

    Ok(connect_to)
}

fn get_track_name(track: &Track) -> &str {
    track
        .info
        .as_ref()
        .map(|i| i.title.as_str())
        .unwrap_or_else(|| track.track.as_str())
}
