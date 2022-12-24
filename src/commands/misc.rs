use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    futures::future::join_all,
    model::prelude::Message,
    prelude::Context,
};

use crate::utils::{guild_ext::GuildExt, log::LogExt};

#[group]
#[commands(att)]
struct Misc;

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
