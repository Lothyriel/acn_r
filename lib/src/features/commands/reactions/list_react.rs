use poise::{command, serenity_prelude::MessageBuilder};

use crate::extensions::serenity::{context_ext::ContextExt, CommandResult, Context};

#[command(prefix_command, guild_only, slash_command, category = "Reactions")]
pub async fn list_react(ctx: Context<'_>) -> CommandResult {
    let reaction_repository = &ctx.data().repositories.reaction;

    let reactions = reaction_repository
        .list(ctx.assure_guild_context()?.0)
        .await?;

    let mut message_builder = MessageBuilder::new();

    message_builder.push_line("Reações: ");

    for reaction in reactions {
        message_builder.push_line(format!("- {reaction}"));
    }

    ctx.say(message_builder.build()).await?;

    Ok(())
}
