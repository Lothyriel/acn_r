use std::sync::Arc;

use anyhow::Error;

use crate::{
    application::models::entities::user::Activity, extensions::serenity::guild_ext::GuildExt,
    features::events::handlers::voice_updated::DispatchData,
};

pub async fn handler(dispatch_data: Arc<DispatchData>) -> Result<(), Error> {
    if dispatch_data.user_id == dispatch_data.bot_id {
        return Ok(());
    }

    if dispatch_data.activity != Activity::Disconnected {
        return Ok(());
    }

    let voice_states = dispatch_data
        .guild_id
        .get_voice_states(dispatch_data.cache.to_owned())?;

    let state = match voice_states.get(&dispatch_data.user_id) {
        Some(v) => v,
        None => return Ok(()),
    };

    let channel = match state.channel_id {
        Some(c) => c,
        None => return Ok(()),
    };

    let someone_online = voice_states.values().any(|v| v.channel_id == Some(channel));

    if !someone_online {
        let lava = dispatch_data.get_lavalink_ctx().await;
        lava.stop_player().await?;
    }

    Ok(())
}
