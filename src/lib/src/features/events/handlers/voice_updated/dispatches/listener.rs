use std::{collections::HashMap, sync::Arc};

use anyhow::Error;
use songbird::CoreEvent;

use crate::{
    extensions::serenity::guild_ext::GuildExt,
    features::events::handlers::voice_updated::DispatchData,
};

pub async fn handler(data: Arc<DispatchData>) -> Result<(), Error> {
    if data.bot_id == data.user_id {
        return Ok(());
    }

    let states = data.guild_id.get_voice_states(data.cache.to_owned())?;

    let possible_channel = states
        .iter()
        .filter_map(|(id, voice)| voice.channel_id.map(|c| (id, c)))
        .fold(HashMap::new(), |mut map, e| {
            let entry = map.entry(e.1).or_insert(0);

            *entry += 1;

            map
        })
        .into_iter()
        .max_by_key(|(_, count)| *count);

    let channel = match possible_channel {
        Some((c, _)) => c,
        None => return disconnect(data).await,
    };

    if let Some(voice_state) = states.get(&data.bot_id) {
        if Some(channel) == voice_state.channel_id {
            return Ok(());
        }
    }

    let (call, result) = data.songbird.join(data.guild_id, channel).await;

    result?;

    {
        let mut handler = call.lock().await;

        handler.add_global_event(CoreEvent::SpeakingStateUpdate.into(), data.to_receiver());

        handler.add_global_event(CoreEvent::SpeakingUpdate.into(), data.to_receiver());

        handler.add_global_event(CoreEvent::VoicePacket.into(), data.to_receiver());

        handler.add_global_event(CoreEvent::ClientDisconnect.into(), data.to_receiver());
    }

    Ok(())
}

async fn disconnect(data: Arc<DispatchData>) -> Result<(), Error> {
    let has_handler = data.songbird.get(data.guild_id).is_some();

    if has_handler {
        data.songbird.remove(data.guild_id).await?;
    }

    Ok(())
}
