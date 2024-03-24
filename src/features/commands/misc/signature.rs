use poise::command;

use crate::{
    application::models::entities::user::Signature,
    extensions::serenity::{CommandResult, Context},
};

#[command(prefix_command, slash_command, category = "Misc")]
pub async fn set_signature(
    ctx: Context<'_>,
    #[rest]
    #[description = "Emojis para assinatura"]
    emojis: String,
) -> CommandResult {
    let user = &ctx.data().repositories.user;

    let signature = Signature {
        date: chrono::Utc::now(),
        user_id: ctx.author().id.get(),
        emojis: emojis.to_owned(),
    };

    user.add_signature(signature).await?;

    ctx.say(format!("Assinatura adicionada: \"{}\"", emojis))
        .await?;

    Ok(())
}
