use std::sync::Arc;

use anyhow::{anyhow, Error};
use chrono_tz::Tz;
use log::error;
use poise::async_trait;
use poise::serenity_prelude::{ChannelId, Guild, GuildId, User};
use songbird::Songbird;

use crate::{
    application::{infra::lavalink_ctx::LavalinkCtx, models::dto::user::GuildInfo},
    extensions::{serenity::Context, std_ext::JoinString},
};

#[async_trait]
pub trait ContextExt {
    async fn get_author_name(self) -> String;
    async fn get_command_args(self) -> String;
    async fn get_lavalink(self) -> Result<LavalinkCtx, Error>;
    async fn assure_connected(self) -> Result<Option<ChannelId>, Error>;
    async fn get_user(self, user_id: u64) -> Result<User, Error>;
    fn get_guild_info(self) -> Option<GuildInfo>;
    fn assure_cached_guild(self) -> Result<Guild, Error>;
    fn assure_guild_context(self) -> Result<GuildId, Error>;
    fn get_time_zone(self) -> Tz;
}

#[async_trait]
impl ContextExt for Context<'_> {
    async fn get_author_name(self) -> String {
        self.author_member()
            .await
            .map(|a| a.display_name().to_string())
            .unwrap_or_else(|| self.author().name.to_owned())
    }

    async fn get_command_args(self) -> String {
        match self {
            poise::Context::Application(ctx) => {
                let args = ctx.args.iter().flat_map(|a| {
                    a.value
                        .as_ref()
                        .map(|v| format!("{v}").trim_matches('"').to_owned())
                });

                args.join(" ")
            }
            poise::Context::Prefix(ctx) => ctx.args.to_owned(),
        }
    }

    async fn get_lavalink(self) -> Result<LavalinkCtx, Error> {
        let guild_id = self.assure_guild_context()?.0;

        let lava_client = self.data().services.lava_client.to_owned();

        let jukebox_repository = self.data().repositories.jukebox.to_owned();

        let user_id = self.author().id.0;

        let songbird = self.data().services.songbird.to_owned();

        Ok(LavalinkCtx::new(
            guild_id,
            user_id,
            songbird,
            lava_client,
            jukebox_repository,
        ))
    }

    async fn assure_connected(self) -> Result<Option<ChannelId>, Error> {
        let guild = self.assure_cached_guild()?;

        let channel = guild
            .voice_states
            .get(&self.author().id)
            .and_then(|voice_state| voice_state.channel_id);

        Ok(channel)
    }

    async fn get_user(self, user_id: u64) -> Result<User, Error> {
        let cached_user = self.serenity_context().cache.user(user_id);

        let user = match cached_user {
            Some(u) => Ok(u),
            None => self.http().get_user(user_id).await,
        };

        Ok(user?)
    }

    fn assure_cached_guild(self) -> Result<Guild, Error> {
        let guild_id = self.assure_guild_context()?;
        self.guild()
            .ok_or_else(|| anyhow!("Couldn't get Guild {} from cache", guild_id))
    }

    fn assure_guild_context(self) -> Result<GuildId, Error> {
        self.guild_id()
            .ok_or_else(|| anyhow!("Context doesn't include an Guild"))
    }

    fn get_guild_info(self) -> Option<GuildInfo> {
        let guild_id = self.guild_id().map(|g| g.0);
        let guild_name = self.guild_id().and_then(|g| g.name(self));

        guild_id.and_then(|i| {
            guild_name.map(|n| GuildInfo {
                guild_id: i,
                guild_name: n,
            })
        })
    }

    fn get_time_zone(self) -> Tz {
        let locale = match self.locale() {
            Some(l) => l,
            None => return chrono_tz::UTC,
        };

        match locale {
            "pt-BR" => chrono_tz::Tz::Brazil__East,
            _ => {
                error!("Encontrada timezone não cadastrada {}", locale);
                chrono_tz::UTC
            }
        }
    }
}

pub async fn get_songbird_client(
    ctx: &poise::serenity_prelude::Context,
) -> Result<Arc<Songbird>, Error> {
    songbird::get(ctx)
        .await
        .ok_or_else(|| anyhow!("Couldn't get songbird voice client"))
}
