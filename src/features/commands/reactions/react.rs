use poise::{command, serenity_prelude::AttachmentType};

use crate::extensions::serenity::serenity_structs::{CommandResult, Context};

#[command(prefix_command, slash_command, category = "reactions")]
pub async fn react(
    ctx: Context<'_>,
    #[description = "Describes the images emotion"] emotion: String,
) -> CommandResult {
    let reaction_repository = &ctx.data().repositories.reaction;

    let reaction = reaction_repository
        .get_reaction(emotion, ctx.guild_id().map(|g| g.0))
        .await?;

    let file = AttachmentType::Bytes {
        data: reaction.bytes.into(),
        filename: reaction.file_name,
    };

    ctx.send(|x| x.attachment(file).content(reaction.emotion))
        .await?;

    Ok(())
}
