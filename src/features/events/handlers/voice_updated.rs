use anyhow::{anyhow, Error};
use serenity::{model::voice::VoiceState, prelude::Context};

use crate::{
    application::{models::{entities::user::Activity, dto::user_services::UpdateActivityDto}, services::mongo::user_services::UserServices},
    extensions::dependency_ext::Dependencies,
};

pub async fn handler(ctx: Context, old: Option<VoiceState>, new: VoiceState) -> Result<(), Error> {
    let now = chrono::Utc::now();

    let activity = match old {
        Some(old_activity) => get_activity(&old_activity, &new),
        None => Activity::Connected,
    };

    let user_services = ctx.get_dependency::<UserServices>().await?;

    let user = new
        .member
        .ok_or_else(|| anyhow!("VoiceStateUpdate não contem membro"))?;

    let guild = ctx.http.get_guild(user.guild_id.0).await?;
    
    let nickname = user.display_name().into_owned();

    let dto = UpdateActivityDto {
        user_id: new.user_id.0,
        guild_id: user.guild_id.0,
        nickname,
        guild_name: guild.name,
        activity: activity,
        date: now,
    };
    user_services.update_user_activity(dto).await?;

    Ok(())
}

fn get_activity(old: &VoiceState, new: &VoiceState) -> Activity {
    if old.channel_id != new.channel_id {
        match new.channel_id {
            Some(new_id) => if let Some(old_id) = old.channel_id {
                if new_id != old_id {
                    return Activity::Moved
                }
                return Activity::Connected
            },
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
