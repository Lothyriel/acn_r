use anyhow::anyhow;
use poise::command;

use crate::extensions::serenity_ext::{CommandResult, Context};

#[command(prefix_command, slash_command, category = "Misc")]
pub async fn debug(ctx: Context<'_>) -> CommandResult {
    let option = {
        let mut configurations = ctx
            .data()
            .app_configurations
            .write()
            .map_err(|_| anyhow!("Failed to get write lock on AppConfigurations"))?;

        configurations.debug = !configurations.debug;

        if configurations.debug {
            "Ligado"
        } else {
            "Desligado"
        }
    };

    ctx.say(format!("O modo debug está {option}")).await?;

    Ok(())
}
