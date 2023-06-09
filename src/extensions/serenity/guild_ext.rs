use std::sync::Arc;

use anyhow::{anyhow, Error};
use serenity::{async_trait, client::Cache, http::Http, model::prelude::GuildId};

use crate::extensions::std_ext::VecResultErrorExt;

#[async_trait]
pub trait GuildExt {
    async fn say_on_main_text_channel(self, http: &Http, msg: &str) -> Result<(), Error>;
    fn get_online_users(self, cache: Arc<Cache>) -> Result<Vec<u64>, Error>;
}

pub trait OptionGuildExt {
    fn assure_guild_context(self) -> Result<GuildId, Error>;
}

impl OptionGuildExt for Option<GuildId> {
    fn assure_guild_context(self) -> Result<GuildId, Error> {
        self.ok_or_else(|| anyhow!("[IMPOSSIBLE] Context doesn't include an Guild"))
    }
}

#[async_trait]
impl GuildExt for GuildId {
    async fn say_on_main_text_channel(self, http: &Http, msg: &str) -> Result<(), Error> {
        let channels = self.channels(&http).await?;

        let channel = channels
            .values()
            .filter(|c| c.is_text_based())
            .min_by(|a, b| a.position.cmp(&b.position))
            .ok_or_else(|| anyhow!("Guild {} doesn't contain a text channel", self.0))?;

        channel.say(http, msg).await?;

        Ok(())
    }

    fn get_online_users(self, cache: Arc<Cache>) -> Result<Vec<u64>, Error> {
        let guild = self
            .to_guild_cached(&cache)
            .ok_or_else(|| anyhow!("Couldn't get Guild {} from cache", self.0))?;

        let online_users = guild
            .voice_states
            .into_values()
            .filter(|v| v.channel_id.is_some())
            .map(|c| c.user_id.0)
            .collect();

        Ok(online_users)
    }
}

pub async fn get_all_online_users(http: Arc<Http>, cache: Arc<Cache>) -> Result<Vec<u64>, Error> {
    let guilds_info = http.get_guilds(None, None).await?;

    let get_online_users_results: Vec<_> = guilds_info
        .into_iter()
        .map(|g| g.id.get_online_users(cache.to_owned()))
        .collect();

    let all_online_users = get_online_users_results.all_successes()?;

    Ok(all_online_users.into_iter().flatten().collect())
}
