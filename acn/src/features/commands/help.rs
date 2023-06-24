use poise::command;

use crate::application::{CommandResult, Context};

#[command(prefix_command, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "O comando específico em questão"] command: Option<String>,
) -> CommandResult {
    let config = poise::builtins::HelpConfiguration {
        extra_text_at_bottom:
            "Se quiser saber algo sobre um comando em específico, passe o nome dele como argumento",
        ..Default::default()
    };

    poise::builtins::help(ctx, command.as_deref(), config).await?;

    Ok(())
}
