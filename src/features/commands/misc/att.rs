use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    futures::future::join_all,
    model::prelude::Message,
    prelude::Context,
};

use crate::extensions::{guild_ext::GuildExt, log_ext::LogErrorsExt};

#[command]
#[owners_only]
#[description("Manda uma mensagem em todos os grupos onde esse bot está presente")]
async fn att(ctx: &Context, _msg: &Message, args: Args) -> CommandResult {
    let message = args.rest();

    let guilds = ctx.http.get_guilds(None, None).await?;

    let tasks: Vec<_> = guilds
        .iter()
        .map(|x| x.id.say_on_main_text_channel(&ctx.http, message))
        .collect();

    join_all(tasks).await.log_errors();

    Ok(())
}
