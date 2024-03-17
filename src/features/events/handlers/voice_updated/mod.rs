use std::sync::Arc;

use anyhow::{anyhow, Error};
use futures::{future::join_all, TryFutureExt};
use poise::serenity_prelude::{Cache, ChannelId, Context, GuildId, Http, UserId, VoiceState};
use songbird::Songbird;

use crate::{
    application::{
        dependency_configuration::DependencyContainer,
        infra::lavalink_ctx::AudioPlayer,
        models::{
            dto::user::{GuildInfo, UserActivityDto},
            entities::user::Activity,
        },
        repositories::jukebox::JukeboxRepository,
    },
    extensions::{log_ext::LogExt, serenity::context_ext::get_songbird_client},
};

mod dispatches;

pub async fn handler(
    ctx: &Context,
    old: &Option<VoiceState>,
    new: &VoiceState,
    data: &DependencyContainer,
) -> Result<(), Error> {
    let user = new.user_id.to_user(ctx).await?;

    let member = new.member.as_ref().ok_or_else(|| {
        anyhow!(
            "{} VoiceStateUpdate triggered outside a Guild context",
            user.name
        )
    })?;

    let guild = ctx.http.get_guild(member.guild_id.get()).await?;

    let activity = match old {
        Some(old_activity) => get_activity(old_activity, new),
        None => Activity::Connected,
    };

    let dto = UserActivityDto {
        user_id: new.user_id.get(),
        guild_info: Some(GuildInfo {
            guild_id: guild.id.get(),
            guild_name: guild.name,
        }),
        nickname: member.display_name().to_string(),
        date: chrono::Utc::now(),
        activity: Some(activity),
    };

    data.repositories.user.update_user(&dto).await?;

    let tasks = vec![
        |c| tokio::spawn(dispatches::songbird_reconnect::handler(c)),
        |c| tokio::spawn(dispatches::songbird_disconnect::handler(c)),
        |c| tokio::spawn(dispatches::afk_disconnect::handler(c)),
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
        cache: ctx.cache.to_owned(),
        http: ctx.http.to_owned(),
        channel_id: new.channel_id,
        user_id: new.user_id,
        jukebox_repository: data.repositories.jukebox.to_owned(),
        bot_id: data.services.bot_id,
        guild_id: member.guild_id,
        songbird: get_songbird_client(ctx),
        activity,
    };

    Ok(dispatch_data)
}

type Tasks = Vec<fn(Arc<DispatchData>) -> tokio::task::JoinHandle<Result<(), Error>>>;

async fn dispatch_tasks(tasks: Tasks, data: Arc<DispatchData>) -> Result<(), Error> {
    let tasks = tasks
        .into_iter()
        .map(|c| c(data.to_owned()).map_err(|e| anyhow!(e)));

    let joins_results = join_all(tasks).await;

    joins_results.into_iter().for_each(|r| match r {
        Ok(s) => s.log(),
        Err(e) => log::error!("{}", e),
    });

    Ok(())
}

pub struct DispatchData {
    cache: Arc<Cache>,
    http: Arc<Http>,
    songbird: Arc<Songbird>,

    jukebox_repository: JukeboxRepository,
    bot_id: UserId,

    user_id: UserId,
    guild_id: GuildId,
    activity: Activity,
    channel_id: Option<ChannelId>,
}

impl DispatchData {
    pub async fn get_player(&self) -> AudioPlayer {
        let jukebox_repository = self.jukebox_repository.to_owned();

        AudioPlayer::new(
            self.guild_id.get(),
            self.user_id.get(),
            self.songbird.to_owned(),
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
