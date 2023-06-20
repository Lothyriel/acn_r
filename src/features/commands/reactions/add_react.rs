use mongodb::bson::oid::ObjectId;
use poise::{command, serenity_prelude::Attachment};

use crate::{
    application::models::{
        dto::reaction_dto::ReactionDto, entities::reaction::Reaction, db_file::DbFile,
    },
    extensions::serenity::serenity_structs::{CommandResult, Context},
};

#[command(prefix_command, slash_command, category = "reactions")]
pub async fn add_react(
    ctx: Context<'_>,
    #[description = "File to examine"] file: Attachment,
    emotion: String,
) -> CommandResult {
    let reaction_repository = &ctx.data().repositories.reaction;

    let reaction = ReactionDto {
        bytes: file.download().await?,
        reaction: Reaction {
            emotion,
            guild_id: ctx.guild_id().map(|f| f.0),
            user_id: ctx.author().id.0,
            file: DbFile {
                filename: file.filename,
                id: ObjectId::new(),
            },
        },
    };

    reaction_repository.add_reaction(reaction).await?;

    ctx.say("salvado").await?;

    Ok(())
}
