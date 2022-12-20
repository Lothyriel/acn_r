use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    http::GuildPagination,
    model::prelude::{GuildId, Message},
    prelude::Context,
};
use crate::utils::http_ext::HttpExt;

#[group]
#[commands(att)]
struct Misc;

#[command]
#[bucket = "pirocudo"]
async fn att(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let message = args.rest();

    // let guild_id = GuildId(244922266050232321);

    // let guilds = ctx.http.get_channels(guild_id).await?;

    let guilds = ctx.http.get_all_guilds();

    msg.reply(ctx, message).await?;

    let cu: Vec<String> = guilds.into_iter().map(|x| x.name).collect();

    msg.reply(ctx, format!("{:?}", cu)).await?;

    Ok(())
}
