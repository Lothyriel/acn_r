use poise::{
    command,
    serenity_prelude::{AttachmentType, Mentionable, UserId},
};

use crate::extensions::serenity::{CommandResult, Context};

#[command(prefix_command, slash_command, category = "R34")]
pub async fn get_voice(
    ctx: Context<'_>,
    #[description = "User to get voice"] _user: Option<UserId>,
) -> CommandResult {
    let voice = &ctx.data().repositories.voice;

    let snippet = match voice.get_voice_snippet().await? {
        Some(s) => s,
        None => {
            ctx.say("Tem nada não...").await?;
            return Ok(());
        }
    };

    let mention = UserId(snippet.user_id).mention();

    let file = AttachmentType::Bytes {
        data: snippet.voice_data.bytes.into(),
        filename: format!("{}_{}", mention, snippet.date),
    };

    ctx.send(|x| {
        x.attachment(file)
            .content(format!("{} : {}", mention, snippet.date))
    })
    .await?;

    Ok(())
}
