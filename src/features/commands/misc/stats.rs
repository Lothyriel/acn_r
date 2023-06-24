use std::cmp::max;

use anyhow::Error;
use futures::future::join_all;
use poise::{
    command,
    serenity_prelude::{GuildId, MessageBuilder, User},
};

use crate::extensions::{
    log_ext::LogErrorsExt,
    serenity::{
        context_ext::ContextExt,
        {CommandResult, Context},
    },
};

const SECONDS_IN_HOUR: i64 = 60 * 60;

#[command(guild_only, prefix_command, slash_command, category = "Misc")]
pub async fn stats(
    ctx: Context<'_>,
    #[description = "Usuário para filtrar estatísticas"] target: Option<User>,
) -> CommandResult {
    let repository = &ctx.data().repositories.stats;

    let guild_id = ctx.assure_guild_context()?;

    let guild_stats = repository
        .get_guild_stats(guild_id.0, target.map(|f| f.id.0))
        .await?;

    let timespan = chrono::Utc::now() - guild_stats.initial_date;
    let total_days = max(timespan.num_days(), 1) as f64;

    let build_message_lines_tasks = guild_stats.stats.into_iter().take(10).map(|f| async move {
        let name = get_name(guild_id, ctx, f.user_id).await?;
        let seconds_online = f.seconds_online;
        let hours_online = seconds_online / SECONDS_IN_HOUR;
        let avg = hours_online as f64 / total_days;

        Ok(format!(
            "- {} ficou {} segundos online ({} hora(s)) - Uma média de {:.2} hora(s) por dia",
            name, seconds_online, hours_online, avg
        ))
    });

    let lines = join_all(build_message_lines_tasks).await.log_errors();

    let mut message_builder = MessageBuilder::new();

    message_builder.push_line(format!(
        "Dados coletados desde: {}",
        guild_stats.initial_date
    ));

    message_builder.push_line("Top 10: ");

    for line in lines {
        message_builder.push_line(line);
    }

    ctx.say(message_builder.build()).await?;

    Ok(())
}

async fn get_name(guild_id: GuildId, ctx: Context<'_>, user_id: u64) -> Result<String, Error> {
    let member_result = guild_id.member(ctx, user_id).await;
    let user_repository = &ctx.data().repositories.user;

    match member_result {
        Ok(m) => Ok(m.display_name().into_owned()),
        Err(_) => Ok(user_repository
            .get_last_name(user_id)
            .await?
            .unwrap_or_else(|| format!("Unknown {user_id}"))),
    }
}
