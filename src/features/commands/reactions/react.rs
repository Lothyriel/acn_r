use poise::{command, serenity_prelude::AttachmentType};

use crate::extensions::serenity::serenity_structs::{CommandResult, Context};

#[command(prefix_command, slash_command, category = "reactions")]
pub async fn react(
    ctx: Context<'_>,
    #[description = "Describes the images emotion"] 
    emotion: String
) -> CommandResult {
    let img = emotion.as_bytes().to_owned(); 
    let sexo = AttachmentType::Bytes { data: img.into(), filename: emotion };

    ctx.send(|x| x.attachment(sexo)).await?;

    Ok(())
}