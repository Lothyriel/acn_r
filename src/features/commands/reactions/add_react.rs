use poise::{command, serenity_prelude::Attachment};

use crate::{extensions::serenity::serenity_structs::{CommandResult, Context}, application::models::entities::reaction::Reaction};

#[command(prefix_command, slash_command, category = "reactions")]
pub async fn add_react(
    ctx: Context<'_>,
    #[description = "File to examine"] 
    file: Attachment,
    emotion: String
) -> CommandResult {
    let service = &ctx.data().reaction_services;

    let reaction = Reaction {
        bytes: file.download().await?,
        emotion,
        file_name: file.filename,
        guild_id: ctx.guild_id().map(|f| f.0),
        user_id: ctx.author().id.0
    };

    service.add_reaction(reaction).await?;
    
    ctx.say("salvado").await?;

    Ok(())
}