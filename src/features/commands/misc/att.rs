use futures::future::join_all;
use poise::{command, serenity_prelude::CacheHttp};

use crate::extensions::{
    log_ext::LogErrorsExt,
    serenity::{
        guild_ext::GuildExt,
        serenity_structs::{CommandResult, Context, OWNERS_ONLY},
    },
};

#[command(
    prefix_command,
    slash_command,
    category = "Misc",
    custom_data = "OWNERS_ONLY"
)]
pub async fn att(
    ctx: Context<'_>,
    #[description = "Mensagem que será enviada as guildas"] message: String,
) -> CommandResult {
    let guilds = ctx.http().get_guilds(None, None).await?;

    let tasks = guilds
        .iter()
        .map(|x| x.id.say_on_main_text_channel(ctx.http(), &message));

    join_all(tasks).await.log_errors();

    Ok(())
}
