use anyhow::Error;
use poise::serenity_prelude::{Context, Message, ReactionType};

use crate::application::dependency_configuration::DependencyContainer;

const REACTIONS_UNICODE: [&str; 8] = ["🇨", "🇺", "♥️", "🇵", "🇷", "🇪", "🇹", "🅾️"];
const ROLE_NAME: &str = "cu-preto";

pub async fn handler(
    ctx: &Context,
    data: &DependencyContainer,
    message: &Message,
) -> Result<(), Error> {
    let guild_id = match message.guild_id {
        Some(g) => g,
        None => return Ok(()),
    };

    let roles = guild_id.roles(ctx).await?;

    let role = match roles.into_values().find(|r| r.name == ROLE_NAME) {
        Some(r) => r,
        None => return Ok(()),
    };

    let has_role = message.author.has_role(&ctx.http, guild_id, role).await?;

    if has_role {
        for unicode_reaction in REACTIONS_UNICODE {
            let reaction = ReactionType::Unicode(unicode_reaction.to_owned());
            message.react(ctx, reaction).await?;
        }
    }

    Ok(())
}
