use poise::command;
use songbird::CoreEvent;

use crate::{
    application::infra::songbird_listener::Receiver,
    extensions::serenity::{
        context_ext::{get_songbird_client, ContextExt},
        CommandResult, Context,
    },
};

#[command(prefix_command, slash_command, guild_only, category = "Listener")]
pub async fn listen(ctx: Context<'_>) -> CommandResult {
    let guild = ctx.assure_guild_context()?;

    let s_ctx = ctx.serenity_context();

    let manager = get_songbird_client(s_ctx).await?;

    let channel = match ctx.assure_connected().await? {
        Some(c) => c,
        None => {
            ctx.say("Join a voice channel").await?;
            return Ok(());
        }
    };

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
