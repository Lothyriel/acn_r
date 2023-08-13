use std::collections::HashMap;

use songbird::CoreEvent;

use crate::{
    application::infra::songbird_listener::Receiver, extensions::serenity::guild_ext::GuildExt,
    features::commands::listener::disconnect,
};

use poise::command;

use crate::extensions::serenity::{
    context_ext::{get_songbird_client, ContextExt},
    CommandResult, Context,
};

#[command(prefix_command, slash_command, guild_only, category = "Listener")]
pub async fn listen(ctx: Context<'_>) -> CommandResult {
    let guild = ctx.assure_guild_context()?;

    let s_ctx = ctx.serenity_context();

    let manager = get_songbird_client(s_ctx).await?;

    let states = guild.get_voice_states(s_ctx.cache.to_owned())?;

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
        None => {
            disconnect(manager.to_owned(), guild).await?;
            return Ok(());
        }
    };

    if let Some(voice_state) = states.get(&ctx.data().services.bot_id) {
        if Some(channel) == voice_state.channel_id {
            return Ok(());
        }
    }

    let (call, result) = manager.join(guild, channel).await;

    result?;

    {
        let mut handler = call.lock().await;

        handler.add_global_event(
            CoreEvent::SpeakingStateUpdate.into(),
            Receiver::from_ctx(&ctx, guild.0),
        );

        handler.add_global_event(
            CoreEvent::SpeakingUpdate.into(),
            Receiver::from_ctx(&ctx, guild.0),
        );

        handler.add_global_event(
            CoreEvent::VoicePacket.into(),
            Receiver::from_ctx(&ctx, guild.0),
        );

        handler.add_global_event(
            CoreEvent::ClientDisconnect.into(),
            Receiver::from_ctx(&ctx, guild.0),
        );
    }

    Ok(())
}
