use poise::command;

use crate::extensions::serenity_ext::{CommandResult, Context};

#[command(prefix_command, slash_command, category = "R34")]
pub async fn random(
    _ctx: Context<'_>,
    #[description = "Prompt to search for"] _search: Option<String>,
) -> CommandResult {
    let _now = chrono::Utc::now();

    Ok(())
}
