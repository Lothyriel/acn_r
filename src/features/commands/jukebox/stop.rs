use poise::command;

use crate::extensions::serenity::{CommandResult, Context, context_ext::ContextExt};

#[command(prefix_command, slash_command, guild_only, category = "Jukebox")]
pub async fn stop(ctx: Context<'_>) -> CommandResult {
    let player = ctx.get_player().await?;

    player.stop(ctx).await
}
