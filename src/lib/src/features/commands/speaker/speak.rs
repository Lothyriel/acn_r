use poise::{
    command,
    serenity_prelude::{AttachmentType, Mentionable, UserId},
};
use songbird::{
    input::{self, cached::Memory, Reader},
    Songbird,
};

use crate::extensions::serenity::{context_ext::ContextExt, CommandResult, Context};

#[command(prefix_command, slash_command, guild_only, category = "Speaker")]
pub async fn speak(
    ctx: Context<'_>,
    #[description = "User to impersonate"] user: Option<UserId>,
) -> CommandResult {
    let voice = &ctx.data().repositories.voice;

    let guild = ctx.assure_guild_context()?;

    let snippet = match voice.get_voice_snippet(guild.0, user.map(|x| x.0)).await? {
        Some(s) => s,
        None => {
            ctx.say("Tem nada não...").await?;
            return Ok(());
        }
    };

    let manager = &ctx.data().services.songbird;

    let channel = match ctx.assure_connected().await? {
        Some(c) => c,
        None => {
            ctx.say("Join a voice channel").await?;
            return Ok(());
        }
    };

    let (call, result) = manager.join(guild, channel).await;

    result?;

    let mut handler = call.lock().await;

    let reader: Reader = snippet.voice_data.bytes.into();

    let audio = input::Input::new(true, reader, input::Codec::Pcm, input::Container::Raw, None);

    handler.play_source(audio);

    Ok(())
}
