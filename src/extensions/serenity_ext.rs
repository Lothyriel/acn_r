use anyhow::{anyhow, Error};
use futures::{future::join_all, Future};
use serenity::{async_trait, http::Http, model::prelude::GuildId};

use crate::application::{
    models::dto::user_services::GuildInfo, services::dependency_configuration::DependencyContainer,
};

#[async_trait]
pub trait GuildExt {
    async fn say_on_main_text_channel(self, http: &Http, msg: &str) -> Result<(), Error>;
}

#[async_trait]
impl GuildExt for GuildId {
    async fn say_on_main_text_channel(self, http: &Http, msg: &str) -> Result<(), Error> {
        let channels = self.channels(&http).await?;

        let channel = channels
            .values()
            .filter(|c| c.is_text_based())
            .min_by(|a, b| a.position.cmp(&b.position))
            .ok_or_else(|| anyhow!("Guild {} doesn't contain a text channel", self.0))?;

        channel.say(http, msg).await?;

        Ok(())
    }
}

#[async_trait]
pub trait ContextExt {
    async fn discord_debug(self, error: &'_ str) -> Result<(), Error>;
    async fn get_author_name(self) -> String;
    async fn get_command_args(self) -> String;
    fn get_guild_info(self) -> Option<GuildInfo>;
}

#[async_trait]
impl ContextExt for Context<'_> {
    async fn discord_debug(self, error: &'_ str) -> Result<(), Error> {
        let debug = {
            let configuration = self
                .data()
                .app_configurations
                .read()
                .map_err(|_| anyhow!("Failed to get read lock on AppConfigurations"))?;

            configuration.debug
        };

        if debug {
            self.say(error).await?;
        }

        Ok(())
    }

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
                            .to_owned()
                            .map(|v| format!("{v}").trim_matches('"').to_owned())
                    })
                    .collect();

                args.join(" ")
            }
            poise::Context::Prefix(ctx) => ctx.args.to_owned(),
        }
    }

    fn get_guild_info(self) -> Option<GuildInfo> {
        let guild_id = self.guild_id().map(|g| g.0);
        let guild_name = self.guild_id().and_then(|g| g.name(self));

        if let Some(id) = guild_id {
            if let Some(name) = guild_name {
                return Some(GuildInfo {
                    guild_id: id,
                    guild_name: name,
                });
            }
        }
        None
    }
}

#[async_trait]
pub trait ErrorExt<T: Send> {
    async fn join_all(self) -> Vec<Result<T, Error>>;
}

#[async_trait]
impl<T: Send, F> ErrorExt<T> for Vec<F>
where
    F: Future<Output = Result<T, serenity::Error>> + Send,
{
    async fn join_all(self) -> Vec<Result<T, Error>> {
        join_all(self)
            .await
            .into_iter()
            .map(|t| t.map_err(|e| anyhow!(e)))
            .collect()
    }
}

pub const OWNERS_ONLY: bool = true;
pub type Context<'a> = poise::Context<'a, DependencyContainer, Error>;
pub type Command = poise::Command<DependencyContainer, Error>;
pub type CommandResult = Result<(), Error>;
pub type FrameworkContext<'a> = poise::FrameworkContext<'a, DependencyContainer, Error>;
pub type FrameworkError<'a> = poise::FrameworkError<'a, DependencyContainer, Error>;
