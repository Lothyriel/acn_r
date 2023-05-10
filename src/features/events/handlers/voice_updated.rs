use std::{ops::Deref, sync::Arc};

use anyhow::{anyhow, Error};
use poise::serenity_prelude::Context;
use serenity::{
    http::{CacheHttp, Http},
    model::{
        prelude::{ChannelId, Member},
        voice::VoiceState,
    },
};

use crate::{
    application::{
        models::{dto::user_services::UpdateActivityDto, entities::user::Activity}, dependency_configuration::DependencyContainer,
    },
    extensions::log_ext::LogExt,
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
    let user = new.user_id.to_user(ctx).await?;
    let member = new.member.as_ref().ok_or_else(|| {
        anyhow!(
            "{} VoiceStateUpdate triggered outside a Guild context",
            user.name
        )
    })?;

    let nickname = member.display_name().to_string();
    let guild_id = member.guild_id.0;

    dispatch_disconnect(guild_id, new, ctx, member);

    let guild = ctx.http().get_guild(guild_id).await?;
    
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
    let cache = ctx.cache.to_owned();

    tokio::spawn(async move {
        services.try_deploy(http, cache).await.log();
    });
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
