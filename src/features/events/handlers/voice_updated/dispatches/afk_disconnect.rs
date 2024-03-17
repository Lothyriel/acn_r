use std::sync::Arc;

use anyhow::Result;

use crate::features::events::handlers::voice_updated::DispatchData;

pub async fn handler(data: Arc<DispatchData>) -> Result<()> {
    disconnect(data).await?;
    Ok(())
}

async fn disconnect(data: Arc<DispatchData>) -> Result<bool> {
    let guild = data.http.get_guild(data.guild_id.get()).await?;

    if guild.afk_channel_id == data.channel_id {
        guild
            .id
            .disconnect_member(data.http.to_owned(), data.user_id)
            .await?;

        return Ok(true);
    }

    Ok(false)
}
