use anyhow::anyhow;
use futures::future::join_all;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
    prelude::{Context, Mentionable},
    utils::MessageBuilder,
};

use crate::{
    application::{ services::stats_services::StatsServices},
    extensions::{dependency_ext::Dependencies, log_ext::LogErrorsExt},
};

#[command]
#[owners_only]
#[only_in(guilds)]
#[description("Mostra os stats dos membros deste server")]
async fn stats(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let service = ctx.get_dependency::<StatsServices>().await?;

    let guild_id = msg
        .guild_id
        .ok_or_else(|| anyhow!("Mensagem não foi enviada de uma guilda (Não deveria ocorrer)"))?;

    let time_online_by_user = service.get_stats_of_guild(guild_id.0).await?;

    let build_message_lines_tasks: Vec<_> = time_online_by_user
        .into_iter()
        .map(|f| async move {
            let user = guild_id.member(&ctx.http, f.0).await?;

            Ok(format!("- {} ficou {} segundos online ({} horas)", user.mention(), f.1, f.1 / 60 / 60))
        })
        .collect();

    let lines = join_all(build_message_lines_tasks).await.log_errors();

    let mut message_builder = MessageBuilder::new();

    for line in lines {
        message_builder.push_line(line);
    }

    msg.reply(&ctx.http, message_builder.build()).await?;

    Ok(())
}
