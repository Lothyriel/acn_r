use poise::command;

use crate::extensions::{
    serenity::context_ext::ContextExt,
    serenity::{CommandResult, Context},
};

#[command(prefix_command, slash_command, guild_only, category = "Jukebox")]
pub async fn playlist(
    ctx: Context<'_>,
    #[rest]
    #[description = "A song URL or YouTube search query"]
    query: String,
) -> CommandResult {
    let songbird = ctx.get_lavalink().await?;

    songbird.playlist(ctx, query).await
}
