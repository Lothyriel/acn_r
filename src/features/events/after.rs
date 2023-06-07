use anyhow::Error;

use crate::{
    application::models::dto::command_use::CommandUseDto,
    extensions::{
        log_ext::LogExt,
        serenity::{context_ext::ContextExt, serenity_structs::Context},
    },
};

async fn after(ctx: Context<'_>) -> Result<(), Error> {
    let now = chrono::Utc::now();

    let command_services = &ctx.data().command_services;

    let guild_info = ctx.get_guild_info();

    let nickname = ctx.get_author_name().await;

    let command_name = ctx.command().name.to_owned();

    let dto = CommandUseDto {
        date: now,
        guild_info,
        user_id: ctx.author().id.0,
        user_nickname: nickname,
        command: command_name,
        args: ctx.get_command_args().await,
    };

    command_services.add_command_use(dto).await?;

    Ok(())
}

pub async fn handler(ctx: Context<'_>) {
    after(ctx).await.log();
}
