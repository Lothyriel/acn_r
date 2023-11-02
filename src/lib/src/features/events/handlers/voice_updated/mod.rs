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
        models::{
            dto::user::{GuildInfo, UpdateUserDto},
            entities::user::Activity,
        },
        repositories::jukebox::JukeboxRepository,
    },
    extensions::std_ext::join_errors,
};

mod dispatches;

pub async fn handler(
    ctx: &Context,
    old: &Option<VoiceState>,
    new: &VoiceState,
    data: &DependencyContainer,
) -> Result<(), Error> {
    let user = new.user_id.to_user(ctx).await?;

    if user.bot {
        return Ok(());
    }

    let member = new.member.as_ref().ok_or_else(|| {
        anyhow!(
            "{} VoiceStateUpdate triggered outside a Guild context",
            user.name
        )
    })?;

    let guild = ctx.http.get_guild(member.guild_id.0).await?;

    let dto = UpdateUserDto {
        user_id: new.user_id.0,
        guild_info: Some(GuildInfo {
            guild_id: guild.id.0,
            guild_name: guild.name,
        }),
        nickname: member.display_name().to_string(),
        date: chrono::Utc::now(),
    };

    data.repositories.user.update_user(dto).await?;

    let tasks = vec![
        |c| tokio::spawn(dispatches::songbird_reconnect::handler(c)),
        |c| tokio::spawn(dispatches::songbird_disconnect::handler(c)),
        |c| tokio::spawn(dispatches::deploy::handler(c)),
        |c| tokio::spawn(dispatches::deploy::handler(c)),
    ];

    let dispatch_data = get_dispatch_data(old, new, ctx, data).await?;

    dispatch_tasks(tasks, Arc::new(dispatch_data)).await
}

async fn get_dispatch_data(
    old: &Option<VoiceState>,
    new: &VoiceState,
    ctx: &Context,
    data: &DependencyContainer,
) -> Result<DispatchData, Error> {
    let activity = match old {
        Some(old_activity) => get_activity(old_activity, new),
        None => Activity::Connected,
    };

    let user = new.user_id.to_user(ctx).await?;

    let member = new.member.as_ref().ok_or_else(|| {
        anyhow!(
            "{} VoiceStateUpdate triggered outside a Guild context",
            user.name
        )
    })?;

    let dispatch_data = DispatchData {
        songbird: data.services.songbird.to_owned(),
        cache: ctx.cache.to_owned(),
        http: ctx.http.to_owned(),
        channel_id: new.channel_id,
        user_id: new.user_id,
        jukebox_repository: data.repositories.jukebox.to_owned(),
        deploy_services: data.services.deploy_services.to_owned(),
        lava_client: data.services.lava_client.to_owned(),
        bot_id: data.services.bot_id,
        guild_id: member.guild_id,
        activity,
    };

    Ok(dispatch_data)
}

type Tasks = Vec<fn(Arc<DispatchData>) -> tokio::task::JoinHandle<Result<(), Error>>>;

async fn dispatch_tasks(tasks: Tasks, data: Arc<DispatchData>) -> Result<(), Error> {
    let tasks = tasks
        .into_iter()
        .map(|c| c(data.to_owned()).map_err(|e| anyhow!(e)));

    let join_results = join_all(tasks).await;

    let a = join_errors(join_results)?;

    _ = join_errors(a)?;

    Ok(())
}

pub struct DispatchData {
    cache: Arc<Cache>,
    http: Arc<Http>,
    songbird: Arc<Songbird>,
    lava_client: LavalinkClient,

    jukebox_repository: JukeboxRepository,
    deploy_services: DeployServices,
    bot_id: UserId,

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
