use poise::command;

use crate::extensions::{
    serenity::context_ext::ContextExt,
    serenity::{CommandResult, Context},
};

#[command(prefix_command, slash_command, guild_only)]
pub async fn queue(ctx: Context<'_>) -> CommandResult {
    let songbird = ctx.get_lavalink().await?;

    songbird.show_queue(ctx).await?;

    Ok(())
}
