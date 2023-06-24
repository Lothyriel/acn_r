use poise::command;

use crate::extensions::{
    serenity::context_ext::ContextExt,
    serenity::{CommandResult, Context},
};

#[command(prefix_command, slash_command, guild_only)]
pub async fn play(
    ctx: Context<'_>,
    #[rest]
    #[description = "A song URL or YouTube search query"]
    query: String,
) -> CommandResult {
    let songbird = ctx.get_lavalink().await?;

    songbird.play(ctx, query).await
}
