use anyhow::Error;
use poise::serenity_prelude::PartialGuild;

use crate::application::dependency_configuration::DependencyContainer;

pub async fn handler(new: &PartialGuild, data: &DependencyContainer) -> Result<(), Error> {
    data.repositories
        .guild
        .add_guild(new.id.0, new.name.to_owned(), chrono::Utc::now())
        .await?;

    Ok(())
}
