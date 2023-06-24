use poise::command;

use crate::extensions::serenity::{
    context_ext::ContextExt,
    {CommandResult, Context},
};

#[command(prefix_command, slash_command, guild_only)]
pub async fn stop(ctx: Context<'_>) -> CommandResult {
    let songbird = ctx.get_lavalink().await?;

    songbird.stop(ctx).await?;

    Ok(())
}
