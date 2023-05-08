use anyhow::{anyhow, Error};

use crate::{
    application::models::dto::command_dto::CommandUseDto,
    extensions::{
        log_ext::LogExt,
        serenity_ext::{ContextExt, FrameworkError},
    },
};

async fn error(err: FrameworkError<'_>) -> Result<(), Error> {
    let now = chrono::Utc::now();

    match err {
        poise::FrameworkError::Command { error, ctx } => {
            let dto = CommandUseDto {
                date: now,
                command: ctx.command().name.to_string(),
                user_id: ctx.author().id.0,
                guild_info: ctx.get_guild_info(),
                user_nickname: ctx.get_author_name().await,
                args: ctx.get_command_args().await,
            };

            let command_services = &ctx.data().command_services;
            let message = format!("{}: {}", ctx.id(), error);
            ctx.discord_debug(&message).await?;

            command_services
                .add_command_error(dto, message.to_string())
                .await?;

            Err(anyhow!("{message}"))
        }
        poise::FrameworkError::EventHandler { error, event, .. } => Err(anyhow!(
            "EventHandler returned error during {:?} event: {:?}",
            event.name(),
            error
        )),
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                Err(anyhow!("Error while handling error: {}", e))
            } else {
                Ok(())
            }
        }
    }
}

pub async fn handler(err: FrameworkError<'_>) {
    error(err).await.log();
}
