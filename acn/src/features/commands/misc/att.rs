use futures::future::join_all;
use lib::extensions::{serenity::{OWNERS_ONLY, guild_ext::GuildExt}, log_ext::LogErrorsExt};
use poise::{command, serenity_prelude::CacheHttp};

use crate::application::{CommandResult, Context};

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

    let errors_count = join_all(tasks).await.log_errors().errors_count;

    ctx.say(format!("Message sent to {} guilds", guilds.len() - errors_count))
        .await?;

    Ok(())
}
