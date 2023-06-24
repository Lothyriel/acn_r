use anyhow::{anyhow, Error};
use lib::{application::models::dto::command_use::CommandUseDto, extensions::{serenity::context_ext::ContextExt, log_ext::LogExt}};

use crate::application::{Context, FrameworkError};

async fn error(err: FrameworkError<'_>) -> Result<(), Error> {
    match err {
        poise::FrameworkError::Command { error, ctx } => handle_command_error(ctx, error).await,
        poise::FrameworkError::EventHandler { error, event, .. } => Err(anyhow!(
            "EventHandler returned error during {} event: {}",
            event.name(),
            error
        )),
        error => poise::builtins::on_error(error)
            .await
            .map_err(|e| anyhow!("Error while handling error: {}", e)),
    }
}

async fn handle_command_error(ctx: Context<'_>, error: Error) -> Result<(), Error> {
    let dto = CommandUseDto {
        date: chrono::Utc::now(),
        command: ctx.command().name.to_owned(),
        user_id: ctx.author().id.0,
        guild_info: ctx.get_guild_info(),
        user_nickname: ctx.get_author_name().await,
        args: ctx.get_command_args(),
    };

    let command_repository = &ctx.data().repositories.command;

    let id = ctx
        .guild()
        .map(|g| g.id.to_string())
        .unwrap_or_else(|| format!("DM: {}", dto.user_id));

    let message = format!("{id}: {error}");

    ctx.say(&message).await?;

    command_repository
        .add_command_error(dto, message.to_owned())
        .await?;

    Err(anyhow!(message))
}

pub async fn handler(err: FrameworkError<'_>) {
    error(err).await.log();
}
