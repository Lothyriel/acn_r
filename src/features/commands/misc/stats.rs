use anyhow::anyhow;
use futures::future::join_all;
use poise::{command, serenity_prelude::User};
use serenity::utils::MessageBuilder;

use crate::extensions::{
    log_ext::LogErrorsExt,
    serenity::serenity_structs::{CommandResult, Context},
};

const SECONDS_IN_HOUR: i64 = 60 * 60;

#[command(guild_only, prefix_command, slash_command, category = "Misc")]
pub async fn stats(
    ctx: Context<'_>,
    #[description = "Usuário para filtrar estatísticas"] target: Option<User>,
) -> CommandResult {
    let service = &ctx.data().stats_services;

    let guild_id = ctx
        .guild_id()
        .ok_or_else(|| anyhow!("Context doesn't include an Guild"))?;

    let guild_stats = service
        .get_guild_stats(guild_id.0, target.map(|f| f.id.0), ctx)
        .await?;

    let build_message_lines_tasks = guild_stats.stats.into_iter().map(|f| async move {
        let member = guild_id.member(ctx, f.user_id).await?;
        let seconds_online = f.seconds_online;
        let hours_online = seconds_online / SECONDS_IN_HOUR;

        Ok(format!(
            "- {} ficou {} segundos online ({} horas)",
            member.display_name(),
            seconds_online,
            hours_online
        ))
    });

    let lines = join_all(build_message_lines_tasks).await.log_errors();

    let mut message_builder = MessageBuilder::new();
    message_builder.push_line(format!(
        "Dados coletados desde: {}",
        guild_stats.initial_date
    ));

    for line in lines {
        message_builder.push_line(line);
    }

    ctx.say(message_builder.build()).await?;

    Ok(())
}
