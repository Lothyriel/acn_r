use poise::command;
use serenity::{futures::future::join_all, http::CacheHttp};

use crate::extensions::{
    log_ext::LogErrorsExt,
    serenity_ext::{CommandResult, Context, GuildExt, OWNERS_ONLY},
};

#[command(prefix_command, slash_command, category = "Misc", custom_data = "OWNERS_ONLY")]
pub async fn att(
    ctx: Context<'_>,
    #[description = "Prompt to search for"] message: String,
) -> CommandResult {
    let guilds = ctx.http().get_guilds(None, None).await?;

    let tasks: Vec<_> = guilds
        .iter()
        .map(|x| x.id.say_on_main_text_channel(ctx.http(), &message))
        .collect();

    join_all(tasks).await.log_errors();

    Ok(())
}
