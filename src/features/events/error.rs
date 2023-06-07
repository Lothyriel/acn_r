use anyhow::{anyhow, Error};

use crate::{
    application::models::dto::command_use::CommandUseDto,
    extensions::{
        log_ext::LogExt,
        serenity::{serenity_structs::{Context, FrameworkError}, context_ext::ContextExt},
    },
};

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
        args: ctx.get_command_args().await,
    };

    let command_services = &ctx.data().command_services;

    let message = format!("{}: {}", ctx.id(), error);

    ctx.say(&message).await?;

    command_services
        .add_command_error(dto, message.to_owned())
        .await?;

    Err(anyhow!("{}", message))
}

pub async fn handler(err: FrameworkError<'_>) {
    error(err).await.log();
}
