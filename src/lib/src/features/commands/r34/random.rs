use poise::command;

use crate::extensions::serenity::{CommandResult, Context};

#[command(prefix_command, slash_command, category = "R34")]
pub async fn random(
    ctx: Context<'_>,
    #[rest]
    #[description = "Prompt to search for"] 
    search: Option<String>,
) -> CommandResult {
    let _now = chrono::Utc::now();

    let client = &ctx.data().services.r34_client;
    let message = client.random(search).await?;

    ctx.say(message).await?;

    Ok(())
}
