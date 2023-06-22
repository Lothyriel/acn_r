use poise::command;

use crate::extensions::serenity::{
    context_ext::ContextExt,
    serenity_structs::{CommandResult, Context},
};

#[command(prefix_command, slash_command, guild_only)]
pub async fn skip(ctx: Context<'_>) -> CommandResult {
    let songbird = ctx.get_lavalink().await?;

    songbird.skip(ctx).await?;

    Ok(())
}
