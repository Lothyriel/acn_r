use anyhow::{anyhow, Error};
use serenity::async_trait;

use crate::application::{infra::songbird::ContextSongbird, models::dto::user::GuildInfo};

use crate::extensions::serenity::serenity_structs::Context;

#[async_trait]
pub trait ContextExt {
    async fn get_author_name(self) -> String;
    async fn get_command_args(self) -> String;
    async fn get_songbird<'a>(self) -> Result<ContextSongbird<'a>, Error>;
    fn get_guild_info(self) -> Option<GuildInfo>;
}

#[async_trait]
impl ContextExt for Context<'_> {
    async fn get_author_name(self) -> String {
        self.author_member()
            .await
            .map(|a| a.display_name().to_string())
            .unwrap_or_else(|| self.author().name.to_owned())
    }

    async fn get_command_args(self) -> String {
        match self {
            poise::Context::Application(ctx) => {
                let args: Vec<_> = ctx
                    .args
                    .into_iter()
                    .flat_map(|a| {
                        a.value
                            .to_owned()
                            .map(|v| format!("{v}").trim_matches('"').to_owned())
                    })
                    .collect();

                args.join(" ")
            }
            poise::Context::Prefix(ctx) => ctx.args.to_owned(),
        }
    }

    async fn get_songbird<'a>(self) -> Result<ContextSongbird<'a>, Error> {
        let guild_id = self.guild_id().assure_guild_context()?.0;

        let songbird = songbird::get(self.serenity_context())
            .await
            .ok_or_else(|| anyhow!("Couldn't get songbird voice client"))?;

        Ok(ContextSongbird {
            ctx: self,
            songbird,
            guild_id,
        })
    }

    fn get_guild_info(self) -> Option<GuildInfo> {
        let guild_id = self.guild_id().map(|g| g.0);
        let guild_name = self.guild_id().and_then(|g| g.name(self));

        guild_id.and_then(|i| {
            guild_name.and_then(|n| {
                Some(GuildInfo {
                    guild_id: i,
                    guild_name: n,
                })
            })
        })
    }
}
