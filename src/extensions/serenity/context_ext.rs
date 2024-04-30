use std::sync::Arc;

use anyhow::{anyhow, Result};
use poise::async_trait;
use poise::serenity_prelude::{ChannelId, GuildId, GuildRef};
use songbird::Songbird;

use crate::{
    application::{infra::audio::player::AudioPlayer, models::dto::user::GuildInfo},
    extensions::{serenity::Context, std_ext::JoinString},
};

#[async_trait]
pub trait ContextExt {
    async fn get_author_name(&self) -> String;
    async fn get_command_args(&self) -> String;
    async fn get_player(&self) -> Result<AudioPlayer>;
    async fn assure_connected(&self) -> Result<Option<ChannelId>>;
    fn get_guild_info(&self) -> Option<GuildInfo>;
    fn assure_cached_guild(&self) -> Result<GuildRef<'_>>;
    fn assure_guild_context(&self) -> Result<GuildId>;
}

#[async_trait]
impl ContextExt for Context<'_> {
    async fn get_author_name(&self) -> String {
        self.author_member()
            .await
            .map(|a| a.display_name().to_string())
            .unwrap_or_else(|| self.author().name.to_owned())
    }

    async fn get_command_args(&self) -> String {
        match self {
            poise::Context::Prefix(ctx) => ctx.args.to_owned(),
            poise::Context::Application(ctx) => ctx
                .args
                .iter()
                .map(|v| format!("{:?}", v).trim_matches('"').to_owned())
                .join(" "),
        }
    }

    async fn get_player(&self) -> Result<AudioPlayer> {
        let guild_id = self.assure_guild_context()?;

        let user_id = self.author().id;

        let songbird = get_songbird_client(self.serenity_context()).await?;

        let manager = self.data().services.audio_manager.clone();

        let jukebox = self.data().repositories.jukebox.clone();

        Ok(AudioPlayer::new(
            guild_id, user_id, manager, jukebox, songbird,
        ))
    }

    async fn assure_connected(&self) -> Result<Option<ChannelId>> {
        let guild = self.assure_cached_guild()?;

        let channel = guild
            .voice_states
            .get(&self.author().id)
            .and_then(|voice_state| voice_state.channel_id);

        Ok(channel)
    }

    fn assure_cached_guild(&self) -> Result<GuildRef<'_>> {
        let guild_id = self.assure_guild_context()?;

        self.guild()
            .ok_or_else(|| anyhow!("Couldn't get Guild {} from cache", guild_id))
    }

    fn assure_guild_context(&self) -> Result<GuildId> {
        self.guild_id()
            .ok_or_else(|| anyhow!("Context doesn't include an Guild"))
    }

    fn get_guild_info(&self) -> Option<GuildInfo> {
        let guild_id = self.guild_id().map(|g| g.get());
        let guild_name = self.guild_id().and_then(|g| g.name(self));

        guild_id.and_then(|i| {
            guild_name.map(|n| GuildInfo {
                guild_id: i,
                guild_name: n,
            })
        })
    }
}

pub async fn get_songbird_client(ctx: &poise::serenity_prelude::Context) -> Result<Arc<Songbird>> {
    songbird::get(ctx)
        .await
        .ok_or_else(|| anyhow!("Couldn't get songbird voice client"))
}
