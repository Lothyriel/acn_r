use poise::{command, serenity_prelude::Attachment};

use crate::{extensions::serenity::serenity_structs::{CommandResult, Context}, application::models::entities::reaction::Reaction};

#[command(prefix_command, slash_command, category = "reactions")]
pub async fn add_react(
    ctx: Context<'_>,
    #[description = "File to examine"] 
    file: Attachment,
) -> CommandResult {
    let service = &ctx.data().reaction_services;

    let reaction = Reaction {
        bytes: file.download().await?,
        emotion
    };

    service.add_reaction(file);
    
    Ok(())
}