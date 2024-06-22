use anyhow::{anyhow, Result};
use poise::{
    async_trait,
    serenity_prelude::{GuildId, Http},
};

#[async_trait]
pub trait GuildExt {
    async fn say_on_main_text_channel(&self, http: &Http, msg: &str) -> Result<()>;
}

#[async_trait]
impl GuildExt for GuildId {
    async fn say_on_main_text_channel(&self, http: &Http, msg: &str) -> Result<()> {
        let channels = self.channels(&http).await?;

        let channel = channels
            .values()
            .filter(|c| c.is_text_based())
            .min_by(|a, b| a.position.cmp(&b.position))
            .ok_or_else(|| anyhow!("Guild {} doesn't contain a text channel", self.get()))?;

        channel.say(http, msg).await?;

        Ok(())
    }
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
