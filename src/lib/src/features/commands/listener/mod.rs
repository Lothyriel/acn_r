use std::sync::Arc;

use anyhow::Error;
use poise::serenity_prelude::GuildId;
use songbird::Songbird;

use crate::extensions::serenity::Command;

mod listen;
mod privacy;

pub fn group() -> Vec<Command> {
    vec![listen::listen(), privacy::privacy()]
}

pub async fn disconnect(songbird: Arc<Songbird>, guild_id: GuildId) -> Result<bool, Error> {
    let has_handler = songbird.get(guild_id).is_some();

    if has_handler {
        songbird.remove(guild_id).await?;
    }

    Ok(has_handler)
}
