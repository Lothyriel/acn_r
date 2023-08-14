use anyhow::Error;
use poise::command;
use poise::serenity_prelude::GuildId;
use songbird::Songbird;

use crate::extensions::serenity::{context_ext::ContextExt, CommandResult, Context};

#[command(prefix_command, slash_command, guild_only, category = "Listener")]
pub async fn privacy(ctx: Context<'_>) -> CommandResult {
    let guild = ctx.assure_guild_context()?;

    let disconnected = disconnect(&ctx.data().services.songbird, guild).await?;

    if !disconnected {
        ctx.say("Not in a voice channel").await?;
    }

    Ok(())
}

pub async fn disconnect(songbird: &Songbird, guild_id: GuildId) -> Result<bool, Error> {
    let has_handler = songbird.get(guild_id).is_some();

    if has_handler {
        songbird.remove(guild_id).await?;
    }

    Ok(has_handler)
}
