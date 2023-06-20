use poise::{command, serenity_prelude::Attachment};

use crate::{
    application::models::entities::reaction::Reaction,
    extensions::serenity::serenity_structs::{CommandResult, Context},
};

#[command(prefix_command, slash_command, category = "reactions")]
pub async fn add_react(
    ctx: Context<'_>,
    #[description = "File to examine"] file: Attachment,
    emotion: String,
) -> CommandResult {
    let reaction_repository = &ctx.data().repositories.reaction;

    let reaction = Reaction {
        bytes: file.download().await?,
        emotion,
        file_name: file.filename,
        guild_id: ctx.guild_id().map(|f| f.0),
        user_id: ctx.author().id.0,
    };

    reaction_repository.add_reaction(reaction).await?;

    ctx.say("salvado").await?;

    Ok(())
}
