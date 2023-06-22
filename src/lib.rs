use anyhow::Error;
use application::dependency_configuration::DependencyContainer;
use extensions::serenity::context_ext;
use poise::serenity_prelude::GatewayIntents;
use songbird::{driver::DecodeMode, Config, SerenityInit};

use crate::{
    application::infra::{
        self,
        appsettings::{self, AppSettings},
        env,
    },
    features::{
        commands::groups_configuration,
        events::{after, check, error, handlers::invoker},
    },
};

pub mod application;
pub mod extensions;
pub mod features;

pub async fn start_application() -> Result<(), Error> {
    let settings = init_app()?;
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

                let songbird = context_ext::get_songbird_client(ctx).await?;

                DependencyContainer::build(
                    settings,
                    ready.user.id.0,
                    ctx.http.to_owned(),
                    ctx.cache.to_owned(),
                    songbird
                )
                .await
            })
        });

    framework.run().await?;

    Ok(())
}

pub fn init_app() -> Result<AppSettings, Error> {
    env::init()?;
    appsettings::load()
}
