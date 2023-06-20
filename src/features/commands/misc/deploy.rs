use anyhow::Error;
use poise::command;

use crate::extensions::serenity::serenity_structs::{CommandResult, Context, OWNERS_ONLY};

#[command(
    prefix_command,
    slash_command,
    category = "Misc",
    custom_data = "OWNERS_ONLY"
)]
pub async fn deploy(ctx: Context<'_>) -> CommandResult {
    let option = {
        let mut configurations = ctx.data().services.app_configurations.write().await;

        configurations.deploy_ready = !configurations.deploy_ready;

        if configurations.deploy_ready {
            "Ativado"
        } else {
            "Desativado"
        }
    };

    ctx.say(format!("Deploy {option}")).await?;

    try_deploy(ctx).await?;

    Ok(())
}

async fn try_deploy(ctx: Context<'_>) -> Result<(), Error> {
    let s_ctx = ctx.serenity_context();
    let deploy_services = &ctx.data().services.deploy_services;

    deploy_services
        .try_deploy(s_ctx.http.to_owned(), s_ctx.cache.to_owned())
        .await
}
