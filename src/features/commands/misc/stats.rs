use anyhow::anyhow;
use chrono::Duration;
use futures::future::join_all;
use itertools::Itertools;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
    prelude::Context,
    utils::MessageBuilder,
};

use crate::{
    application::{
        models::entities::{user::Activity, user_activity::UserActivity},
        services::stats_services::StatsServices,
    },
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

    let a = service.get_stats_by_guild(guild_id.0).await;

    if let Err(e) = &a {
        print!("{e}");
    }

    let guild_stats = a?;

    let stats_by_user = guild_stats.into_iter().group_by(|e| e.user_id);

    let time_online_by_user: Vec<_> = stats_by_user
        .into_iter()
        .map(|e| get_online_time(e.0, e.1.collect_vec()))
        .collect();

    let build_message_lines_tasks: Vec<_> = time_online_by_user
        .into_iter()
        .map(|f| async move {
            let user = guild_id.member(&ctx.http, f.0).await?;

            Ok(format!("{} spent {} online", user.display_name(), f.1))
        })
        .collect();

    let lines = join_all(build_message_lines_tasks).await.log_errors();

    let mut message_builder = MessageBuilder::new();

    for line in lines {
        message_builder.push(line);
    }

    msg.reply(&ctx.http, message_builder.build()).await?;

    Ok(())
}

fn get_online_time(user_id: u64, activities: Vec<UserActivity>) -> (u64, Duration) {
    let connects: Vec<_> = activities
        .iter()
        .filter(|e| e.activity_type == Activity::Connected)
        .collect();

    let disconnects: Vec<_> = activities
        .iter()
        .filter(|e| e.activity_type == Activity::Disconnected)
        .collect();

    let zip = connects.into_iter().zip(disconnects);

    let connected_seconds: Vec<_> = zip
        .into_iter()
        .map(|e| {
            let connected = e.0.date;
            let disconnected = e.1.date;

            let time = connected - disconnected;

            time.num_seconds()
        })
        .collect();

    let total_seconds_connected = connected_seconds.into_iter().sum();

    (user_id, Duration::seconds(total_seconds_connected))
}
