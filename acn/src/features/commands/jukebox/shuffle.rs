use poise::command;

use crate::application::{CommandResult, Context, AppContextExt};

#[command(prefix_command, slash_command, guild_only, category = "Jukebox")]
pub async fn shuffle(ctx: Context<'_>) -> CommandResult {
    let songbird = ctx.get_lavalink().await?;

    songbird.shuffle(ctx).await?;

    Ok(())
}
