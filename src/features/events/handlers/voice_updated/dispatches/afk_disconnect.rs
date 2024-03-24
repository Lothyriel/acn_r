use std::sync::Arc;

use anyhow::Result;

use crate::features::events::handlers::voice_updated::DispatchData;

pub async fn handler(data: Arc<DispatchData>) -> Result<()> {
    disconnect(data).await?;
    Ok(())
}

async fn disconnect(data: Arc<DispatchData>) -> Result<bool> {
    let guild = data.http.get_guild(data.guild_id).await?;

    let guild_afk_channel = match guild.afk_metadata.map(|g| g.afk_channel_id) {
        Some(c) => c,
        None => return Ok(false),
    };

    let channel = match data.channel_id {
        Some(c) => c,
        None => return Ok(false),
    };

    if guild_afk_channel == channel {
        guild
            .id
            .disconnect_member(data.http.to_owned(), data.user_id)
            .await?;

        return Ok(true);
    }

    Ok(false)
}
