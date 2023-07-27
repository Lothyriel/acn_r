use poise::command;

use crate::extensions::serenity::{
    context_ext::{get_songbird_client, ContextExt},
    CommandResult, Context,
};

#[command(prefix_command, slash_command, guild_only, category = "Listener")]
pub async fn privacy(ctx: Context<'_>) -> CommandResult {
    let guild = ctx.assure_guild_context()?;

    let manager = get_songbird_client(ctx.serenity_context()).await?;

    let has_handler = manager.get(guild).is_some();

    match has_handler {
        true => {
            manager.remove(guild).await?;
        }
        false => {
            ctx.say("Not in a voice channel").await?;
        }
    }

    Ok(())
}