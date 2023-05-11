use poise::command;

use crate::extensions::serenity_ext::{CommandResult, Context, OWNERS_ONLY};

#[command(
    prefix_command,
    slash_command,
    category = "Misc",
    custom_data = "OWNERS_ONLY"
)]
pub async fn deploy(ctx: Context<'_>) -> CommandResult {
    let option = {
        let mut configurations = ctx
            .data()
            .app_configurations
            .write()
            .await;

        configurations.deploy_ready = !configurations.deploy_ready;

        if configurations.deploy_ready {
            "Ativado"
        } else {
            "Desativado"
        }
    };

    ctx.say(format!("Deploy {option}")).await?;

    Ok(())
}
