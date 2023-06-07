use serenity::async_trait;

use crate::application::models::dto::user::GuildInfo;

use super::serenity_structs::Context;

#[async_trait]
pub trait ContextExt {
    async fn get_author_name(self) -> String;
    async fn get_command_args(self) -> String;
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

    fn get_guild_info(self) -> Option<GuildInfo> {
        let guild_id = self.guild_id().map(|g| g.0);
        let guild_name = self.guild_id().and_then(|g| g.name(self));

        if let Some(id) = guild_id {
            if let Some(name) = guild_name {
                return Some(GuildInfo {
                    guild_id: id,
                    guild_name: name,
                });
            }
        }
        None
    }
}
