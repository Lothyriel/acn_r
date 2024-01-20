use anyhow::anyhow;
use poise::{command, serenity_prelude::User};

use crate::{
    extensions::serenity::{CommandResult, Context},
    features::commands::stats::send_stats,
};

#[command(guild_only, prefix_command, slash_command, category = "Stats")]
pub async fn stats(
    ctx: Context<'_>,
    #[description = "Usuário para filtrar estatísticas"] target: Option<User>,
) -> CommandResult {
    let service = &ctx.data().repositories.stats;

    let guild_id = ctx
        .guild_id()
        .ok_or_else(|| anyhow!("Context doesn't include an Guild"))?;

    let guild_stats = service
        .get_guild_stats(guild_id.0, target.map(|f| f.id.0))
        .await?;

    send_stats(ctx, guild_stats, guild_id).await
}
