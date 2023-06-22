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

    let online_users = dispatch_data
        .guild_id
        .get_online_users(dispatch_data.cache.to_owned())?;

    if online_users.is_empty() {
        let lava = dispatch_data.get_lavalink_ctx().await;
        lava.stop_player().await?;
    }

    Ok(())
}
