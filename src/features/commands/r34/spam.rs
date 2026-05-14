use futures::future::try_join_all;
use poise::{command, serenity_prelude::User};

use crate::extensions::serenity::{CommandResult, Context};

use super::random::send_hentai;

#[command(prefix_command, slash_command, category = "R34", user_cooldown = 1800)]
pub async fn spam(
    ctx: Context<'_>,
    #[description = "Usuário alvo"] target: Option<User>,
    #[description = "Tag opcional"]
    #[rest]
    search: Option<String>,
) -> CommandResult {
    let tasks = (1..15).map(|_| send_hentai(ctx, target.as_ref(), search.as_deref()));

    try_join_all(tasks).await?;

    Ok(())
}
