use poise::{
    command,
    serenity_prelude::{AttachmentType, Mentionable, UserId},
};

use crate::extensions::serenity::{context_ext::ContextExt, CommandResult, Context};

#[command(prefix_command, slash_command, guild_only, category = "Speaker")]
pub async fn get_voice(
    ctx: Context<'_>,
    #[description = "User to get voice"] _user: Option<UserId>,
) -> CommandResult {
    let voice = &ctx.data().repositories.voice;

    let guild = ctx.assure_guild_context()?;

    let snippet = match voice.get_voice_snippet(guild.0).await? {
        Some(s) => s,
        None => {
            ctx.say("Tem nada não...").await?;
            return Ok(());
        }
    };

    let user_id = ctx.get_user(snippet.user_id).await?;

    let file = AttachmentType::Bytes {
        data: snippet.voice_data.bytes.into(),
        filename: format!("{}_{}.wav", user_id.name, snippet.date.timestamp()),
    };

    let time_zone = ctx.get_time_zone();

    ctx.send(|x| {
        x.attachment(file).content(format!(
            "{} : {}",
            user_id.mention(),
            snippet.date.with_timezone(&time_zone).naive_local()
        ))
    })
    .await?;

    Ok(())
}
