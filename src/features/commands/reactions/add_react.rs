use std::io::Cursor;

use poise::{command, serenity_prelude::Attachment};

use crate::{
    application::models::dto::reaction_dto::AddReactionDto,
    extensions::serenity::{context_ext::ContextExt, CommandResult, Context},
};

#[command(prefix_command, guild_only, slash_command, category = "reactions")]
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
        emotion: emotion.to_lowercase(),
        guild_id: ctx.assure_guild_context()?.0,
        user_id: ctx.author().id.0,
        filename: file.filename,
    };

    reaction_repository.add_reaction(dto).await?;

    ctx.say(format!("Registrada emoção: \"{}\"", emotion.to_lowercase()))
        .await?;

    Ok(())
}
