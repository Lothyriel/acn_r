use std::{collections::HashSet, sync::Arc};

use anyhow::{anyhow, Error};
use poise::{
    async_trait,
    serenity_prelude::{Cache, GuildId, Http},
};

use crate::{
    application::models::entities::{user::Activity, user_activity::UserActivity},
    extensions::std_ext::VecResultErrorExt,
};

#[async_trait]
pub trait GuildExt {
    async fn say_on_main_text_channel(self, http: &Http, msg: &str) -> Result<(), Error>;
    fn get_online_users(self, cache: Arc<Cache>) -> Result<HashSet<StatusInfo>, Error>;
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
        let guild = cache.guild(self).ok_or_else(|| anyhow!(""))?;

        let online_users = guild
            .voice_states
            .into_values()
            .filter(|v| v.channel_id.is_some())
            .map(|c| StatusInfo::new(c.user_id.0, guild.id.0))
            .collect();

        Ok(online_users)
    }
}

pub async fn get_all_online_users(
    http: Arc<Http>,
    cache: Arc<Cache>,
) -> Result<HashSet<StatusInfo>, Error> {
    let guilds_info = http.get_guilds(None, None).await?;

    let get_online_users_results: Vec<_> = guilds_info
        .into_iter()
        .map(|g| g.id.get_online_users(cache.to_owned()))
        .collect();

    let all_online_users = get_online_users_results.all_successes()?;

    Ok(all_online_users.into_iter().flatten().collect())
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

    pub fn to_activity(&self, activity: Activity) -> UserActivity {
        UserActivity {
            guild_id: self.guild_id,
            user_id: self.user_id,
            date: chrono::Utc::now(),
            activity_type: activity,
        }
    }
}
