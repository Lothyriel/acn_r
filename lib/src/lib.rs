use anyhow::Error;
use application::infra::appsettings::AppSettings;
use poise::serenity_prelude::GatewayIntents;
use songbird::{driver::DecodeMode, Config, SerenityInit};

use crate::{
    application::{
        dependency_configuration::DependencyContainer,
        infra::{self, appsettings, env},
    },
    features::{
        commands::acn_commands,
        events::{after, check, error, handlers::invoker},
    },
};

pub mod application;
pub mod extensions;
pub mod features;

pub async fn start_acn() -> Result<(), Error> {
    let settings = get_settings()?;
    let token = env::get("TOKEN_BOT")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            event_handler: |ctx, event, _, user_data| {
                Box::pin(invoker::ready_handler(ctx, event, user_data))
            },
            commands: acn_commands::register_commands(),
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
        .client_settings(|c| c.register_songbird_from_config(Config::default()))
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

pub async fn start_listener() -> Result<(), Error> {
    let settings = get_settings()?;
    let token = env::get("TOKEN_BOT")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: acn_commands::register_commands(),
            event_handler: |ctx, event, _, user_data| {
                Box::pin(invoker::handler(ctx, event, user_data))
            },
            on_error: |error| Box::pin(error::handler(error)),
            post_command: |ctx| Box::pin(after::handler(ctx)),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: None,
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

pub fn get_settings() -> Result<AppSettings, Error> {
    appsettings::load()
}
