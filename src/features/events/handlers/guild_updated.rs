use anyhow::Error;
use serenity::model::prelude::PartialGuild;

use crate::application::dependency_configuration::DependencyContainer;

pub async fn handler(
    new: &PartialGuild,
    data: &DependencyContainer,
) -> Result<(), Error> {
    let now = chrono::Utc::now();
    let guild_services = &data.guild_services;

    guild_services.add_guild(new.id.0, new.name.to_owned(), now).await?;

    Ok(())
}
