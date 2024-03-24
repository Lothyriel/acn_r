use poise::command;

use crate::extensions::serenity::{context_ext::ContextExt, CommandResult, Context};

#[command(prefix_command, slash_command, guild_only, category = "Jukebox")]
pub async fn skip(ctx: Context<'_>) -> CommandResult {
    let player = ctx.get_player().await?;

    player.skip(ctx).await
}
