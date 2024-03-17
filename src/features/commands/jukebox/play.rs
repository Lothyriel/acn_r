use poise::command;

use crate::extensions::{
    serenity::context_ext::ContextExt,
    serenity::{CommandResult, Context},
};

#[command(prefix_command, slash_command, guild_only, category = "Jukebox")]
pub async fn play(
    ctx: Context<'_>,
    #[rest]
    #[description = "A song URL or YouTube search query"]
    query: String,
) -> CommandResult {
    let player = ctx.get_player().await?;

    player.play(ctx, query).await
}
