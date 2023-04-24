use anyhow::Error;
use serenity::{model::prelude::PartialGuild, prelude::Context};

use crate::{
    application::services::guild_services::GuildServices,
    extensions::dependency_ext::Dependencies,
};

pub async fn handler(ctx: Context, new: PartialGuild) -> Result<(), Error> {
    let now = chrono::Utc::now();
    let guild_services = ctx.get_dependency::<GuildServices>().await?;

    guild_services.add_guild(new.id.0, new.name, now).await?;

    Ok(())
}
