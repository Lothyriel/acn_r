use std::{ops::Deref, sync::Arc};

use anyhow::{anyhow, Error};
use chrono::Duration;
use futures::future::join_all;
use poise::serenity_prelude::Context;
use serenity::{
    http::{CacheHttp, Http},
    model::{
        prelude::{ChannelId, Member},
        voice::VoiceState,
    },
};
use tokio::time::sleep;

use crate::{
    application::{
        models::{dto::user_services::UpdateActivityDto, entities::user::Activity},
        services::{
            dependency_configuration::DependencyContainer, github_services::GithubServices,
        },
    },
    extensions::{log_ext::LogExt, std_ext::VecResultErrorExt},
};

pub async fn handler(
    ctx: &Context,
    old: &Option<VoiceState>,
    new: &VoiceState,
    data: &DependencyContainer,
) -> Result<(), Error> {
    let now = chrono::Utc::now();

    let activity = match old {
        Some(old_activity) => get_activity(&old_activity, &new),
        None => Activity::Connected,
    };

    let user_services = &data.user_services;

    dispatch_deploy(ctx, data);

    let member = new
        .member
        .as_ref()
        .ok_or_else(|| anyhow!("VoiceStateUpdate não contem membro"))?;

    let guild_id = member.guild_id.0;

    dispatch_disconnect(guild_id, new, ctx, member);

    let guild = ctx.http().get_guild(guild_id).await?;

    let nickname = member.display_name().to_string();

    let dto = UpdateActivityDto {
        user_id: new.user_id.0,
        guild_id: guild_id,
        guild_name: guild.name,
        nickname,
        activity,
        date: now,
    };

    user_services.update_user_activity(dto).await?;

    Ok(())
}

fn dispatch_deploy(ctx: &Context, data: &DependencyContainer) {
    let http = ctx.http.to_owned();
    let services = data.github_services.to_owned();

    tokio::spawn(async {
        try_deploy(http, services).await.log();
    });
}

async fn try_deploy(http: Arc<Http>, services: GithubServices) -> Result<bool, Error> {
    match is_someone_online(http.to_owned()).await? {
        true => Ok(false),
        false => {
            sleep(Duration::minutes(5).to_std()?).await;
            match is_someone_online(http).await? {
                true => Ok(false),
                false => {
                    services.start_deploy().await?;
                    Ok(true)
                }
            }
        }
    }
}

async fn is_someone_online(http: Arc<Http>) -> Result<bool, Error> {
    let guilds_info = http.get_guilds(None, None).await?;
    let tasks_get_guild: Vec<_> = guilds_info
        .into_iter()
        .map(|g| http.get_guild(g.id.0))
        .collect();

    let get_guild_results: Vec<_> = join_all(tasks_get_guild)
        .await
        .into_iter()
        .map(|t| t.map_err(|e| anyhow!(e)))
        .collect();

    let guilds = get_guild_results.all_successes()?;

    let presence_count_results: Vec<_> = guilds
        .into_iter()
        .map(|g| {
            g.approximate_presence_count
                .ok_or_else(|| anyhow!("Error getting presence count of: {}", g.id))
        })
        .collect();

    let presence_counts = presence_count_results.all_successes()?;
    Ok(presence_counts.iter().any(|p| p > &u64::MIN))
}

fn dispatch_disconnect(guild_id: u64, new: &VoiceState, ctx: &Context, member: &Member) {
    let data = Arc::new(DisconnectData {
        guild_id,
        channel_id: new.channel_id,
        http: ctx.http.to_owned(),
        member: member.deref().to_owned(),
    });

    tokio::spawn(async {
        disconnect_afk(data).await.log();
    });
}

struct DisconnectData {
    guild_id: u64,
    channel_id: Option<ChannelId>,
    http: Arc<Http>,
    member: Member,
}

async fn disconnect_afk(data: Arc<DisconnectData>) -> Result<bool, Error> {
    let guild = data.http.get_guild(data.guild_id).await?;
    if guild.afk_channel_id == data.channel_id {
        data.member.disconnect_from_voice(&data.http).await?;
        return Ok(true);
    }

    Ok(false)
}

fn get_activity(old: &VoiceState, new: &VoiceState) -> Activity {
    if old.channel_id != new.channel_id {
        match new.channel_id {
            Some(new_id) => {
                if let Some(old_id) = old.channel_id {
                    if new_id != old_id {
                        return Activity::Moved;
                    }
                    return Activity::Connected;
                }
            }
            None => return Activity::Disconnected,
        }
    }

    if old.self_stream != new.self_stream {
        match new.self_stream {
            Some(_) => return Activity::OpenedStream,
            None => return Activity::ClosedStream,
        }
    }

    if old.self_video != new.self_video {
        match new.self_video {
            true => return Activity::OpenedCamera,
            false => return Activity::ClosedCamera,
        }
    }

    if old.deaf != new.deaf {
        match new.deaf {
            true => return Activity::Muted,
            false => return Activity::Unmuted,
        }
    }

    if old.mute != new.mute {
        match new.mute {
            true => return Activity::Muted,
            false => return Activity::Unmuted,
        }
    }

    if old.self_deaf != new.self_deaf {
        match new.self_deaf {
            true => return Activity::Muted,
            false => return Activity::Unmuted,
        }
    }

    if old.self_mute != new.self_mute {
        match new.self_mute {
            true => return Activity::Muted,
            false => return Activity::Unmuted,
        }
    }

    Activity::Disconnected
}
