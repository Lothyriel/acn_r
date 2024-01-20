use std::{collections::HashMap, sync::Arc};

use anyhow::{anyhow, Error};
use poise::{
    async_trait,
    serenity_prelude::{Cache, GuildId, Http, UserId, VoiceState},
};

use crate::extensions::std_ext::collapse_errors;

#[async_trait]
pub trait GuildExt {
    async fn say_on_main_text_channel(self, http: &Http, msg: &str) -> Result<(), Error>;
    fn get_voice_states(self, cache: Arc<Cache>) -> Result<HashMap<UserId, VoiceState>, Error>;
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

    fn get_voice_states(self, cache: Arc<Cache>) -> Result<HashMap<UserId, VoiceState>, Error> {
        let guild = cache
            .guild(self)
            .ok_or_else(|| anyhow!("Couldn't get guild {} from cache", self.0))?;

        Ok(guild.voice_states)
    }
}

pub async fn get_all_online_users(
    http: Arc<Http>,
    cache: Arc<Cache>,
) -> Result<impl Iterator<Item = StatusInfo>, Error> {
    let guilds_info = http.get_guilds(None, None).await?;

    let get_online_users_results = guilds_info.into_iter().map(move |g| {
        let voice_states = g.id.get_voice_states(cache.to_owned())?;

        let online_users = voice_states
            .into_values()
            .filter(|v| v.channel_id.is_some())
            .map(move |c| StatusInfo::new(c.user_id.0, g.id.0));

        Ok(online_users)
    });

    let online_users_results = collapse_errors(get_online_users_results)?;

    Ok(online_users_results.into_iter().flatten())
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct StatusInfo {
    pub user_id: u64,
    pub guild_id: u64,
}

impl StatusInfo {
    pub fn new(user_id: u64, guild_id: u64) -> Self {
        Self { user_id, guild_id }
    }
}
