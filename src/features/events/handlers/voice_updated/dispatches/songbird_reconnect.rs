use std::sync::Arc;

use anyhow::Error;

use crate::{
    application::{infra::songbird::SongbirdCtx, models::entities::user::Activity},
    features::events::handlers::voice_updated::DispatchData,
};

pub async fn handler(dispatch_data: Arc<DispatchData>) -> Result<(), Error> {
    if dispatch_data.activity == Activity::Moved {
        dispatch_songbird_reconnect(dispatch_data).await?;
    }

    Ok(())
}

async fn dispatch_songbird_reconnect(dispatch_data: Arc<DispatchData>) -> Result<(), Error> {
    if dispatch_data.user_id != dispatch_data.id {
        return Ok(());
    }

    let channel = match dispatch_data.channel_id {
        Some(id) => id,
        None => return Ok(()),
    };

    let lava_client = dispatch_data.lava_client.to_owned();
    let jukebox_services = dispatch_data.jukebox_services.to_owned();

    let songbird_ctx = SongbirdCtx::new(
        dispatch_data.guild_id,
        dispatch_data.user_id.0,
        dispatch_data.songbird.to_owned(),
        lava_client,
        jukebox_services,
    );

    let _ = songbird_ctx.join_voice_channel(channel).await;
    songbird_ctx.join_voice_channel(channel).await
}