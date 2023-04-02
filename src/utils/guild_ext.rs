use anyhow::{anyhow, Error};
use serenity::{async_trait, http::Http, model::prelude::GuildId};
use std::sync::Arc;

#[async_trait]
pub trait GuildExt {
    async fn say_on_main_text_channel(self, http: &Arc<Http>, msg: &str) -> Result<(), Error>;
}

#[async_trait]
impl GuildExt for GuildId {
    async fn say_on_main_text_channel(self, http: &Arc<Http>, msg: &str) -> Result<(), Error> {
        let channels = self.channels(&http).await?;

        let channel = channels
            .values()
            .filter(|c| c.is_text_based())
            .min_by(|a, b| a.position.cmp(&b.position))
            .ok_or_else(|| anyhow!("Não achei um canal principal {}", self.0))?;

        channel.say(http, msg).await?;

        Ok(())
    }
}
