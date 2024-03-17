use poise::command;

use crate::extensions::{
    serenity::context_ext::ContextExt,
    serenity::{CommandResult, Context},
};

#[command(prefix_command, slash_command, guild_only, category = "Jukebox")]
pub async fn queue(ctx: Context<'_>) -> CommandResult {
    let songbird = ctx.get_player().await?;

    songbird.show_queue(ctx).await?;

    Ok(())
}
