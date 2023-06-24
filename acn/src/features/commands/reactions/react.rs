use lib::extensions::serenity::context_ext::ContextExt;
use poise::{command, serenity_prelude::AttachmentType};

use crate::application::{CommandResult, Context};

#[command(prefix_command, guild_only, slash_command, category = "Reactions")]
pub async fn react(
    ctx: Context<'_>,
    #[rest]
    #[description = "Describes the images emotion"]
    emotion: Option<String>,
) -> CommandResult {
    let reaction_repository = &ctx.data().repositories.reaction;

    let (reaction, bytes) = reaction_repository
        .reaction(emotion, ctx.assure_guild_context()?.0, ctx.author().id.0)
        .await?;

    let file = AttachmentType::Bytes {
        data: bytes.into(),
        filename: reaction.filename,
    };

    ctx.send(|x| x.attachment(file).content(reaction.emotion))
        .await?;

    Ok(())
}
