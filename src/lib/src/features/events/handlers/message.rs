use anyhow::Error;
use poise::serenity_prelude::{Context, Message, ReactionType};

use crate::application::dependency_configuration::DependencyContainer;

pub async fn handler(
    ctx: &Context,
    data: &DependencyContainer,
    message: &Message,
) -> Result<(), Error> {
    let signature = data
        .repositories
        .user
        .get_last_signature(message.author.id.0)
        .await?;

    match signature {
        Some(s) => react(message, ctx, &s.emojis).await?,
        None => (),
    }

    Ok(())
}

async fn react(message: &Message, ctx: &Context, emojis: &str) -> Result<(), Error> {
    for emoji in emojis.chars().filter(|&c| c != ' ') {
        let reaction = ReactionType::Unicode(emoji.to_string());

        message.react(ctx, reaction).await.ok();
    }

    Ok(())
}
