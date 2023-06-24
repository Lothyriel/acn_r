use crate::application::dependency_configuration::DependencyContainer;

pub mod lavalink_ctx;
pub mod dependency_configuration;

pub type Context<'a> = poise::Context<'a, DependencyContainer, Error>;
pub type Command = poise::Command<DependencyContainer, Error>;
pub type CommandResult = Result<(), Error>;
pub type FrameworkContext<'a> = poise::FrameworkContext<'a, DependencyContainer, Error>;
pub type FrameworkError<'a> = poise::FrameworkError<'a, DependencyContainer, Error>;

async fn get_lavalink(ctx: Context<'_>) -> Result<LavalinkCtx, Error> {
    let guild_id = ctx.assure_guild_context()?.0;

    let lava_client = ctx.data().services.lava_client.to_owned();

    let jukebox_repository = ctx.data().repositories.jukebox.to_owned();

    let user_id = ctx.author().id.0;

    let songbird = get_songbird_client(ctx.serenity_context()).await?;

    Ok(LavalinkCtx::new(
        guild_id,
        user_id,
        songbird,
        lava_client,
        jukebox_repository,
    ))
}