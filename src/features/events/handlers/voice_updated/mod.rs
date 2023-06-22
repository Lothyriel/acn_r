use std::sync::Arc;

use anyhow::{anyhow, Error};
use futures::{future::join_all, TryFutureExt};
use lavalink_rs::LavalinkClient;
use poise::serenity_prelude::{Cache, ChannelId, Context, GuildId, Http, UserId, VoiceState};
use songbird::Songbird;

use crate::{
    application::{
        dependency_configuration::DependencyContainer,
        infra::{deploy_service::DeployServices, lavalink_ctx::LavalinkCtx},
        models::{dto::user::UpdateActivityDto, entities::user::Activity},
        repositories::jukebox::JukeboxRepository,
    },
    extensions::{serenity::context_ext, std_ext::VecResultErrorExt},
};

mod dispatches;

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

    let status_monitor = &data.services.status_monitor;

    let user = new.user_id.to_user(ctx).await?;

    let member = new.member.as_ref().ok_or_else(|| {
        anyhow!(
            "{} VoiceStateUpdate triggered outside a Guild context",
            user.name
        )
    })?;

    let nickname = member.display_name().to_string();

    let guild_id = member.guild_id;

    let guild = ctx.http.get_guild(guild_id.0).await?;

    let dto = UpdateActivityDto {
        user_id: new.user_id.0,
        guild_id: guild_id.0,
        guild_name: guild.name,
        nickname,
        activity,
        date: now,
    };

    status_monitor.update_user_activity(dto).await?;

    let dispatch_data = DispatchData {
        songbird: context_ext::get_songbird_client(ctx).await?,
        cache: ctx.cache.to_owned(),
        http: ctx.http.to_owned(),
        channel_id: new.channel_id,
        user_id: new.user_id,
        jukebox_repository: data.repositories.jukebox.to_owned(),
        deploy_services: data.services.deploy_services.to_owned(),
        lava_client: data.services.lava_client.to_owned(),
        bot_id: data.services.bot_id,
        guild_id,
        activity,
    };

    trigger_dispatches(Arc::new(dispatch_data)).await
}

async fn trigger_dispatches(data: Arc<DispatchData>) -> Result<(), Error> {
    let tasks = [
        |c| tokio::spawn(dispatches::afk_disconnect::handler(c)),
        |c| tokio::spawn(dispatches::deploy::handler(c)),
        |c| tokio::spawn(dispatches::songbird_reconnect::handler(c)),
        |c| tokio::spawn(dispatches::songbird_disconnect::handler(c)),
    ]
    .into_iter()
    .map(|c| c(data.to_owned()).map_err(|e| anyhow!(e)));

    let dispatches_results = join_all(tasks).await.all_successes()?;

    dispatches_results.all_successes()?;

    Ok(())
}

pub struct DispatchData {
    cache: Arc<Cache>,
    http: Arc<Http>,
    songbird: Arc<Songbird>,
    lava_client: LavalinkClient,

    jukebox_repository: JukeboxRepository,
    deploy_services: DeployServices,
    bot_id: u64,

    user_id: UserId,
    guild_id: GuildId,
    activity: Activity,
    channel_id: Option<ChannelId>,
}

impl DispatchData {
    pub async fn get_lavalink_ctx(&self) -> LavalinkCtx {
        let lava_client = self.lava_client.to_owned();
        let jukebox_repository = self.jukebox_repository.to_owned();

        LavalinkCtx::new(
            self.guild_id.0,
            self.user_id.0,
            self.songbird.to_owned(),
            lava_client,
            jukebox_repository,
        )
    }
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
