use poise::{command, serenity_prelude::User};

use crate::extensions::serenity::{CommandResult, Context};

use super::random::send_hentai;

#[command(prefix_command, slash_command, category = "R34", user_cooldown = 3)]
pub async fn hentai(
    ctx: Context<'_>,
    #[description = "Usuário alvo"] target: Option<User>,
    #[description = "Tag opcional"]
    #[rest]
    search: Option<String>,
) -> CommandResult {
    send_hentai(ctx, target.as_ref(), search.as_deref()).await
}
