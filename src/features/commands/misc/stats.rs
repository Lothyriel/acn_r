use anyhow::anyhow;
use futures::future::join_all;
use poise::{command, serenity_prelude::User};
use serenity::{prelude::Mentionable, utils::MessageBuilder};

use crate::extensions::{
    log_ext::LogErrorsExt,
    serenity_ext::{CommandResult, Context},
};

#[command(prefix_command, slash_command, category = "Misc")]
pub async fn stats(
    ctx: Context<'_>,
    #[description = "Prompt to search for"] target: Option<User>,
) -> CommandResult {
    let service = &ctx.data().stats_services;

    let guild_id = ctx
        .guild_id()
        .ok_or_else(|| anyhow!("Mensagem não foi enviada de uma guilda (Não deveria ocorrer)"))?;

    let guild_stats = service
        .get_stats_of_guild(guild_id.0, target.map(|f| f.id.0))
        .await?;

    let guild = ctx
        .guild_id()
        .ok_or_else(|| anyhow!("Comando não usado em guilda"))?;

    let build_message_lines_tasks: Vec<_> = guild_stats
        .stats
        .into_iter()
        .map(|f| async move {
            let user = guild.member(ctx, f.user_id).await?;
            let seconds_online = f.seconds_online;
            let hours_online = seconds_online / 60 / 60;

            Ok(format!(
                "- {} ficou {} segundos online ({} horas)",
                user.mention(),
                seconds_online,
                hours_online
            ))
        })
        .collect();

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
