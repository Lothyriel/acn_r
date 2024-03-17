use poise::command;

use crate::extensions::serenity::{CommandResult, Context};

#[command(prefix_command, slash_command, guild_only, category = "Jukebox")]
pub async fn skip(ctx: Context<'_>) -> CommandResult {
    let songbird = ctx.get_player().await?;

    songbird.skip(ctx).await?;

    Ok(())
}
