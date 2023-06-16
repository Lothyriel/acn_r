use poise::command;

use crate::extensions::serenity::{
    context_ext::ContextExt,
    serenity_structs::{CommandResult, Context},
};

#[command(prefix_command, slash_command, guild_only)]
pub async fn shuffle(ctx: Context<'_>) -> CommandResult {
    let songbird = ctx.get_songbird().await?;

    songbird.shuffle(ctx).await?;

    Ok(())
}
