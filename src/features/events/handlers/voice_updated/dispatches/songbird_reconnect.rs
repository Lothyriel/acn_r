use std::sync::Arc;

use anyhow::Error;

use crate::{
    application::models::entities::user::Activity,
    features::events::handlers::voice_updated::DispatchData,
};

pub async fn handler(dispatch_data: Arc<DispatchData>) -> Result<(), Error> {
    if dispatch_data.activity == Activity::Moved {
        dispatch_songbird_reconnect(dispatch_data).await?;
    }

    Ok(())
}

async fn dispatch_songbird_reconnect(dispatch_data: Arc<DispatchData>) -> Result<(), Error> {
    if dispatch_data.user_id != dispatch_data.bot_id {
        return Ok(());
    }

    let channel = match dispatch_data.channel_id {
        Some(id) => id,
        None => return Ok(()),
    };

    let songbird_ctx = dispatch_data.get_lavalink_ctx().await;

    let _ = songbird_ctx.join_voice_channel(channel).await;
    songbird_ctx.join_voice_channel(channel).await
}
