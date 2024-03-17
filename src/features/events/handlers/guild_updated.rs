use anyhow::Result;
use poise::serenity_prelude::PartialGuild;

use crate::application::dependency_configuration::DependencyContainer;

pub async fn handler(new: &PartialGuild, data: &DependencyContainer) -> Result<()> {
    data.repositories
        .guild
        .add_guild(new.id.get(), new.name.as_str(), chrono::Utc::now())
        .await?;

    Ok(())
}
