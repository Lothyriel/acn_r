use anyhow::{anyhow, bail, Result};

use crate::{
    application::models::dto::command_use::CommandUseDto,
    extensions::{
        log_ext::LogExt,
        serenity::{
            context_ext::ContextExt,
            {Context, FrameworkError},
        },
    },
};

async fn error(err: FrameworkError<'_>) -> Result<()> {
    match err {
        FrameworkError::Command { error, ctx, .. } => handle_command_error(ctx, error).await,
        poise::FrameworkError::EventHandler {
            error,
            ctx: _,
            event,
            framework: _,
            ..
        } => bail!(
            "EventHandler returned error during {} event: {}",
            event.snake_case_name(),
            error
        ),
        error => poise::builtins::on_error(error)
            .await
            .map_err(|e| anyhow!("Error while handling error: {}", e)),
    }
}

async fn handle_command_error(ctx: Context<'_>, error: anyhow::Error) -> Result<()> {
    let dto = CommandUseDto {
        date: chrono::Utc::now(),
        command: ctx.command().name.to_owned(),
        user_id: ctx.author().id.get(),
        guild_info: ctx.get_guild_info(),
        user_nickname: ctx.get_author_name().await,
        args: ctx.get_command_args().await,
    };

    let command_repository = &ctx.data().repositories.command;

    let id = ctx
        .guild()
        .map(|g| g.id.to_string())
        .unwrap_or_else(|| format!("DM: {}", dto.user_id));

    let message = format!("{}: {}", id, error);

    ctx.say(&message).await?;

    command_repository
        .add_command_error(dto, message.as_str())
        .await?;

    bail!(message)
}

pub async fn handler(err: FrameworkError<'_>) {
    error(err).await.log();
}
