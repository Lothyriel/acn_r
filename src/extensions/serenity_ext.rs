use anyhow::{anyhow, Error};
use serenity::{async_trait, http::Http, model::prelude::GuildId};

use crate::application::services::dependency_configuration::DependencyContainer;

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
            .ok_or_else(|| anyhow!("Não achei um canal principal {}", self.0))?;

        channel.say(http, msg).await?;

        Ok(())
    }
}

pub type Command = poise::Command<DependencyContainer, Error>;
pub type Context<'a> = poise::Context<'a, DependencyContainer, Error>;
pub type FrameworkContext<'a> = poise::FrameworkContext<'a, DependencyContainer, Error>;
pub type CommandResult = Result<(), Error>;