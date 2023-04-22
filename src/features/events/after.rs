use anyhow::Error;
use serenity::{
    framework::standard::{macros::hook, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

use crate::{
    application::{
        models::dto::{command_dto::CommandUseDto, user_services::GuildInfo},
        services::mongo::command_services::CommandServices,
    },
    extensions::{dependency_ext::Dependencies, log_ext::LogExt},
};

#[hook]
pub async fn after(ctx: &Context, msg: &Message, name: &str, result: CommandResult) {
    handler(ctx, msg, name, result).await.log();
}

async fn handler<'a>(
    ctx: &'a Context,
    msg: &'a Message,
    name: &'a str,
    result: CommandResult,
) -> Result<(), Error> {
    let now = chrono::Utc::now();

    let command_services = ctx.get_dependency::<CommandServices>().await?;

    let guild_info = get_guild_info(msg, ctx);

    let nickname = msg
        .author_nick(&ctx.http)
        .await
        .unwrap_or_else(|| msg.author.name.to_string());

    let command_with_prefix = format!("!{name}");
    let args = msg
        .content
        .replace(&command_with_prefix, "")
        .trim()
        .to_string();

    let dto = CommandUseDto {
        date: now,
        guild_info,
        user_id: msg.author.id.0,
        user_nickname: nickname,
        command: name.to_string(),
        args,
    };

    if let Err(e) = result {
        command_services
            .add_command_error(&dto, format!("{e}"))
            .await?
    }

    command_services.add_command_use(dto).await?;

    Ok(())
}

fn get_guild_info(msg: &Message, ctx: &Context) -> Option<GuildInfo> {
    let guild_id = msg.guild_id.map(|g| g.0);
    let guild_name = msg.guild(&ctx.cache).map(|g| g.name);

    if let Some(id) = guild_id {
        if let Some(name) = guild_name {
            return Some(GuildInfo {
                guild_id: id,
                guild_name: name,
            });
        }
    }
    None
}
