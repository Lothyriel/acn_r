use std::sync::Arc;

use anyhow::Result;

use crate::{
    application::models::entities::user::Activity,
    features::events::handlers::voice_updated::DispatchData,
};

pub async fn handler(dispatch_data: Arc<DispatchData>) -> Result<()> {
    if dispatch_data.user_id != dispatch_data.bot_id {
        return Ok(());
    }

    if dispatch_data.activity != Activity::Moved {
        return Ok(());
    }

    let channel = match dispatch_data.channel_id {
        Some(id) => id,
        None => return Ok(()),
    };

    let player = dispatch_data.get_player();

    //  BUG: não sei por que só funcina chamando duas vezes
    let _ = player.join_voice_channel(channel).await;
    player.join_voice_channel(channel).await
}
