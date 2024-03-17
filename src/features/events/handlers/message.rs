use anyhow::Result;
use poise::serenity_prelude::{Context, Message, ReactionType};

use crate::application::dependency_configuration::DependencyContainer;

pub async fn handler(ctx: &Context, data: &DependencyContainer, message: &Message) -> Result<()> {
    let signature = data
        .repositories
        .user
        .get_last_signature(message.author.id.get())
        .await?;

    if let Some(s) = signature {
        react(message, ctx, &s.emojis).await?
    }

    Ok(())
}

async fn react(message: &Message, ctx: &Context, emojis: &str) -> Result<()> {
    for emoji in emojis.chars().filter(|&c| c != ' ') {
        let reaction = ReactionType::Unicode(emoji.to_string());

        message.react(ctx, reaction).await.ok();
    }

    Ok(())
}
