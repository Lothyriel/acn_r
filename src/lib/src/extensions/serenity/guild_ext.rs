use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use anyhow::{anyhow, Error};
use poise::{
    async_trait,
    serenity_prelude::{Cache, GuildId, Http, UserId, VoiceState},
};

use crate::extensions::std_ext::join_errors;

#[async_trait]
pub trait GuildExt {
    async fn say_on_main_text_channel(self, http: &Http, msg: &str) -> Result<(), Error>;
    fn get_online_users(self, cache: Arc<Cache>) -> Result<HashSet<StatusInfo>, Error>;
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

    fn get_online_users(self, cache: Arc<Cache>) -> Result<HashSet<StatusInfo>, Error> {
        let voice_states = self.get_voice_states(cache)?;

        let online_users = voice_states
            .into_values()
            .filter(|v| v.channel_id.is_some())
            .map(|c| StatusInfo::new(c.user_id.0, self.0))
            .collect();

        Ok(online_users)
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
) -> Result<HashSet<StatusInfo>, Error> {
    let guilds_info = http.get_guilds(None, None).await?;

    let get_online_users_results = guilds_info
        .into_iter()
        .map(|g| g.id.get_online_users(cache.to_owned()));

    let all_online_users = join_errors(get_online_users_results)?;

    Ok(all_online_users.flatten().collect())
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
