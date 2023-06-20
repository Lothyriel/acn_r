use std::sync::Arc;

use anyhow::{anyhow, Error};
use poise::async_trait;
use poise::serenity_prelude::{Guild, GuildId};
use songbird::Songbird;

use crate::{
    application::{infra::lavalink_ctx::LavalinkCtx, models::dto::user::GuildInfo},
    extensions::serenity::serenity_structs::Context,
};

#[async_trait]
pub trait ContextExt {
    async fn get_author_name(self) -> String;
    async fn get_command_args(self) -> String;
    async fn get_songbird(self) -> Result<LavalinkCtx, Error>;
    fn get_guild_info(self) -> Option<GuildInfo>;
    fn assure_cached_guild(self) -> Result<Guild, Error>;
    fn assure_guild_context(self) -> Result<GuildId, Error>;
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
                let args: Vec<_> = ctx
                    .args
                    .into_iter()
                    .flat_map(|a| {
                        a.value
                            .as_ref()
                            .map(|v| format!("{v}").trim_matches('"').to_owned())
                    })
                    .collect();

                args.join(" ")
            }
            poise::Context::Prefix(ctx) => ctx.args.to_owned(),
        }
    }

    async fn get_songbird(self) -> Result<LavalinkCtx, Error> {
        let guild_id = self.assure_guild_context()?.0;

        let lava_client = self.data().lava_client.to_owned();

        let jukebox_services = self.data().jukebox_services.to_owned();

        let user_id = self.author().id.0;

        let songbird = get_songbird_client(self.serenity_context()).await?;

        Ok(LavalinkCtx::new(
            guild_id,
            user_id,
            songbird,
            lava_client,
            jukebox_services,
        ))
    }

    fn assure_cached_guild(self) -> Result<Guild, Error> {
        let guild_id = self.assure_guild_context()?;
        self.guild()
            .ok_or_else(|| anyhow!("Couldn't get Guild {guild_id} from cache"))
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
}

pub async fn get_songbird_client(
    ctx: &poise::serenity_prelude::Context,
) -> Result<Arc<Songbird>, Error> {
    songbird::get(ctx)
        .await
        .ok_or_else(|| anyhow!("Couldn't get songbird voice client"))
}
