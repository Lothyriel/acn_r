use poise::{command, serenity_prelude::MessageBuilder};

use crate::extensions::serenity::serenity_structs::{CommandResult, Context};

#[command(prefix_command, slash_command, category = "reactions")]
pub async fn list_react(ctx: Context<'_>) -> CommandResult {
    let reaction_repository = &ctx.data().repositories.reaction;

    let reactions = reaction_repository
        .list(ctx.guild_id().map(|g| g.0))
        .await?;

    let mut message_builder = MessageBuilder::new();

    message_builder.push_line("Reações: ");

    for reaction in reactions {
        message_builder.push_line(format!("- {reaction}"));
    }

    ctx.say(message_builder.build()).await?;

    Ok(())
}
