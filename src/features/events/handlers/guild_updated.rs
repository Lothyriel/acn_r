use anyhow::Error;
use poise::serenity_prelude::PartialGuild;

use crate::application::dependency_configuration::DependencyContainer;

pub async fn handler(new: &PartialGuild, data: &DependencyContainer) -> Result<(), Error> {
    let now = chrono::Utc::now();
    let guild_repository = &data.repositories.guild;

    guild_repository
        .add_guild(new.id.0, new.name.to_owned(), now)
        .await?;

    Ok(())
}
