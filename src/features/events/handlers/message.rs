use anyhow::Result;
use poise::serenity_prelude::{Context, Message, ReactionType};
use unicode_segmentation::UnicodeSegmentation;

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
    for emoji in emojis
        .split_whitespace()
        .flat_map(|token| token.graphemes(true))
    {
        let reaction = ReactionType::Unicode(emoji.to_owned());

        message.react(ctx, reaction).await.ok();
    }

    Ok(())
}
