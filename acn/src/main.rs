use anyhow::Error;
use application::dependency_configuration::DependencyContainer;
use features::events::{after, check, error, handlers::invoker};
use poise::serenity_prelude::GatewayIntents;
use serde::Deserialize;
use songbird::{driver::DecodeMode, Config, SerenityInit};

use lib::{
    application::infra::{
        appsettings::{self, GithubSettings, LavalinkSettings, MongoSettings},
        env,
    },
    extensions::log_ext::LogExt,
};

use crate::features::commands::groups_configuration;

mod application;
mod features;

#[tokio::main]
async fn main() {
    start_application().await.log()
}

pub async fn start_application() -> Result<(), Error> {
    let settings: AppSettings = appsettings::load()?;
    let token = env::get("TOKEN_BOT")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: groups_configuration::register_commands(),
            event_handler: |ctx, event, _, user_data| {
                Box::pin(invoker::handler(ctx, event, user_data))
            },
            on_error: |error| Box::pin(error::handler(error)),
            post_command: |ctx| Box::pin(after::handler(ctx)),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(settings.prefix.to_owned()),
                mention_as_prefix: true,
                ..Default::default()
            },
            command_check: Some(|ctx| Box::pin(check::handler(ctx))),
            ..Default::default()
        })
        .token(&token)
        .client_settings(|c| {
            c.register_songbird_from_config(Config::default().decode_mode(DecodeMode::Decode))
        })
        .intents(GatewayIntents::all())
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                DependencyContainer::build(
                    settings,
                    ready.user.id,
                    ctx.http.to_owned(),
                    ctx.cache.to_owned(),
                )
                .await
            })
        });

    framework.run().await?;

    Ok(())
}

#[derive(Deserialize)]
pub struct AppSettings {
    pub allowed_ids: Vec<u64>,
    pub prefix: String,
    pub lavalink_settings: LavalinkSettings,
    pub mongo_settings: MongoSettings,
    pub github_settings: GithubSettings,
}
