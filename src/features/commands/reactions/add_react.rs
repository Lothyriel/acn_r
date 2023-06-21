use std::io::Cursor;

use poise::{command, serenity_prelude::Attachment};

use crate::{
    application::models::dto::reaction_dto::AddReactionDto,
    extensions::serenity::serenity_structs::{CommandResult, Context},
};

#[command(prefix_command, slash_command, category = "reactions")]
pub async fn add_react(
    ctx: Context<'_>,
    #[description = "File to examine"] file: Attachment,
    emotion: String,
) -> CommandResult {
    let mut reaction_repository = ctx.data().repositories.reaction.to_owned();

    let now = chrono::Utc::now();

    let dto = AddReactionDto {
        bytes: Cursor::new(file.download().await?),
        date: now,
        emotion,
        guild_id: ctx.guild_id().map(|f| f.0),
        user_id: ctx.author().id.0,
        filename: file.filename,
    };

    reaction_repository.add_reaction(dto).await?;

    ctx.say("salvado").await?;

    Ok(())
}
