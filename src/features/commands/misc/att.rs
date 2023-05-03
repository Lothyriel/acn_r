use std::fmt::format;

use anyhow::anyhow;
use serenity::{
    framework::standard::{macros::command, Args, CommandError, CommandResult},
    futures::future::join_all,
    model::prelude::Message,
    prelude::Context,
};

use crate::extensions::{guild_ext::GuildExt, log_ext::LogErrorsExt};

#[command]
#[owners_only]
#[description("Manda uma mensagem em todos os grupos onde esse bot está presente")]
async fn att(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        let message = format!("O comando precisa de uma mensagem como argumento");
        msg.reply(&ctx.http, message).await?;
        Err(anyhow!("Faltando Argumentos"))?;
    }

    let message = args.rest();

    let guilds = ctx.http.get_guilds(None, None).await?;

    let tasks: Vec<_> = guilds
        .iter()
        .map(|x| x.id.say_on_main_text_channel(&ctx.http, message))
        .collect();

    join_all(tasks).await.log_errors();

    Ok(())
}
