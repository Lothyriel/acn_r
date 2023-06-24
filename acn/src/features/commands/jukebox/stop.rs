use poise::command;

use crate::application::{CommandResult, Context, AppContextExt};

#[command(prefix_command, slash_command, guild_only, category = "Jukebox")]
pub async fn stop(ctx: Context<'_>) -> CommandResult {
    let songbird = ctx.get_lavalink().await?;

    songbird.stop(ctx).await?;

    Ok(())
}
