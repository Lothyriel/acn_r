use poise::command;

use crate::extensions::serenity::{CommandResult, Context};

#[command(prefix_command, slash_command, category = "R34")]
pub async fn random(
    _ctx: Context<'_>,
    #[rest]
    #[description = "Prompt to search for"] 
    _search: Option<String>,
) -> CommandResult {
    let _now = chrono::Utc::now();

    let client = &_ctx.data().services.r34_client;
    client.random_spam().await?;

    Ok(())
}
