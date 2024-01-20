use anyhow::Error;
use futures::future::join_all;
use poise::serenity_prelude::{GuildId, MessageBuilder};

use crate::{
    application::models::dto::stats::{StatsDto, UserStats},
    extensions::{
        serenity::{Command, CommandResult, Context},
        std_ext::collapse_errors,
    },
};

mod stats_1;
mod stats_2;

pub fn group() -> Vec<Command> {
    vec![stats_1::stats(), stats_2::stats2()]
}

const SECONDS_IN_HOUR: i64 = 60 * 60;

pub async fn send_stats(ctx: Context<'_>, g_stats: StatsDto, guild_id: GuildId) -> CommandResult {
    let tasks = g_stats
        .stats
        .into_iter()
        .map(|s| build_message(s, guild_id, ctx));

    let lines = collapse_errors(join_all(tasks).await.into_iter());

    let mut message_builder = MessageBuilder::new();
    message_builder.push_line(format!("Dados coletados desde: {}", g_stats.initial_date));

    for line in lines? {
        message_builder.push_line(line);
    }

    ctx.say(message_builder.build()).await?;

    Ok(())
}

async fn build_message(
    stats: UserStats,
    guild_id: GuildId,
    ctx: Context<'_>,
) -> Result<String, Error> {
    let member = guild_id.member(ctx, stats.user_id).await?;
    let seconds_online = stats.seconds_online;
    let hours_online = seconds_online / SECONDS_IN_HOUR;

    Ok(format!(
        "- {} ficou {} segundos online ({} horas)",
        member.display_name(),
        seconds_online,
        hours_online
    ))
}
