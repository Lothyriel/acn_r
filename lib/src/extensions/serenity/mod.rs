use anyhow::Error;

use crate::application::dependency_configuration::DependencyContainer;

pub mod context_ext;
pub mod guild_ext;

pub const OWNERS_ONLY: bool = true;
pub type Context<'a> = poise::Context<'a, DependencyContainer, Error>;
pub type Command = poise::Command<DependencyContainer, Error>;
pub type CommandResult = Result<(), Error>;
pub type FrameworkContext<'a> = poise::FrameworkContext<'a, DependencyContainer, Error>;
pub type FrameworkError<'a> = poise::FrameworkError<'a, DependencyContainer, Error>;
