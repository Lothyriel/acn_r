use anyhow::anyhow;
use futures::future::join_all;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
    prelude::{Context, Mentionable},
    utils::MessageBuilder,
};

use crate::{
    application::services::stats_services::StatsServices,
    extensions::{dependency_ext::Dependencies, log_ext::LogErrorsExt},
};

#[command]
#[owners_only]
#[only_in(guilds)]
#[description("Mostra os stats dos membros deste server")]
async fn stats(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let target = args.single::<u64>().ok();
    args.restore();
    print!("{:?}", args.current());
    let service = ctx.get_dependency::<StatsServices>().await?;

    let guild_id = msg
        .guild_id
        .ok_or_else(|| anyhow!("Mensagem não foi enviada de uma guilda (Não deveria ocorrer)"))?;

    let guild_stats = service.get_stats_of_guild(guild_id.0, target).await?;

    let build_message_lines_tasks: Vec<_> = guild_stats
        .stats
        .into_iter()
        .map(|f| async move {
            let user = guild_id.member(&ctx.http, f.user_id).await?;
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
    message_builder.push_line(format!("Dados coletados desde: {}", guild_stats.initial_date));

    for line in lines {
        message_builder.push_line(line);
    }

    msg.reply(&ctx.http, message_builder.build()).await?;

    Ok(())
}
