use poise::command;

use crate::{
    extensions::serenity::{
        context_ext::{get_songbird_client, ContextExt},
        CommandResult, Context,
    },
    features::commands::listener::disconnect,
};

#[command(prefix_command, slash_command, guild_only, category = "Listener")]
pub async fn privacy(ctx: Context<'_>) -> CommandResult {
    let guild = ctx.assure_guild_context()?;

    let manager = get_songbird_client(ctx.serenity_context()).await?;

    let disconnected = disconnect(manager.to_owned(), guild).await?;

    if !disconnected {
        ctx.say("Not in a voice channel").await?;
    }

    Ok(())
}
