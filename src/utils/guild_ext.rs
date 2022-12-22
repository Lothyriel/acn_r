use std::sync::Arc;

use serenity::{
    async_trait,
    http::Http,
    model::prelude::{GuildId},
    Error,
};

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
            .min_by(|a, b| a.position.cmp(&b.position))
            .ok_or_else(|| Error::Other("Não achei um canal"))?;

        channel.say(http, msg).await?;

        Ok(())
    }
}
