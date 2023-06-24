use anyhow::Error;
use lavalink_rs::async_trait;
use lib::extensions::serenity::context_ext::{get_songbird_client, ContextExt};

use crate::application::{
    dependency_configuration::DependencyContainer, lavalink_ctx::LavalinkCtx,
};

pub mod dependency_configuration;
pub mod lavalink_ctx;

pub type Context<'a> = poise::Context<'a, DependencyContainer, Error>;
pub type Command = poise::Command<DependencyContainer, Error>;
pub type CommandResult = Result<(), Error>;
pub type FrameworkError<'a> = poise::FrameworkError<'a, DependencyContainer, Error>;

#[async_trait]
pub trait AppContextExt {
    async fn get_lavalink(self) -> Result<LavalinkCtx, Error>;
}

#[async_trait]
impl AppContextExt for Context<'_> {
    async fn get_lavalink(self) -> Result<LavalinkCtx, Error> {
        let guild_id = self.assure_guild_context()?.0;

        let lava_client = self.data().services.lava_client.to_owned();

        let jukebox_repository = self.data().repositories.jukebox.to_owned();

        let user_id = self.author().id.0;

        let songbird = get_songbird_client(self.serenity_context()).await?;

        Ok(LavalinkCtx::new(
            guild_id,
            user_id,
            songbird,
            lava_client,
            jukebox_repository,
        ))
    }
}
