use serenity::{framework::standard::{macros::command, CommandResult, Args}, prelude::Context, model::prelude::Message, futures::future::join_all};

use crate::extensions::{guild_ext::GuildExt, log_ext::LogExt};

#[command]
#[bucket = "pirocudo"]
async fn att(ctx: &Context, _msg: &Message, args: Args) -> CommandResult {
    let message = args.rest();

    let guilds = ctx.http.get_guilds(None, None).await?;

    let futures: Vec<_> = guilds
        .iter()
        .map(|x| x.id.say_on_main_text_channel(&ctx.http, message))
        .collect();

    join_all(futures).await.log();

    Ok(())
}