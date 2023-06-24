use poise::command;

use crate::application::{AppContextExt, CommandResult, Context};

#[command(prefix_command, slash_command, guild_only, category = "Jukebox")]
pub async fn play(
    ctx: Context<'_>,
    #[rest]
    #[description = "A song URL or YouTube search query"]
    query: String,
) -> CommandResult {
    let songbird = ctx.get_lavalink().await?;

    songbird.play(ctx, query).await
}