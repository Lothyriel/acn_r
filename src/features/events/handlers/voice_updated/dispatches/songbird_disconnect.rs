use std::sync::Arc;

use anyhow::{anyhow, Result};

use crate::{
    application::models::entities::user::Activity,
    features::events::handlers::voice_updated::DispatchData,
};

pub async fn handler(dispatch_data: Arc<DispatchData>) -> Result<()> {
    if dispatch_data.user_id == dispatch_data.bot_id {
        return Ok(());
    }

    if dispatch_data.activity != Activity::Disconnected {
        return Ok(());
    }

    let guild = dispatch_data
        .cache
        .guild(dispatch_data.guild_id)
        .ok_or_else(|| anyhow!("Couldn't get guild {} from cache", dispatch_data.guild_id))?;

    let state = match guild.voice_states.get(&dispatch_data.bot_id) {
        Some(v) => v,
        None => return Ok(()),
    };

    let channel = state
        .channel_id
        .ok_or_else(|| anyhow!("Voice state should have a channel id"))?;

    let online_count = guild
        .voice_states
        .values()
        .filter(|v| v.channel_id == Some(channel))
        .count();

    if online_count == 1 {
        let player = dispatch_data.get_player().await;
        player.stop_player().await?;
    }

    Ok(())
}
