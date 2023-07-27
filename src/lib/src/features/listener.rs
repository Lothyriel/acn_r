use anyhow::Error;
use poise::serenity_prelude::GatewayIntents;
use songbird::{driver::DecodeMode, Config, SerenityInit};

use crate::{
    application::{
        dependency_configuration::DependencyContainer,
        infra::{appsettings, env},
    },
    extensions::serenity::Command,
    features::{
        commands::{help, misc::privacy},
        events::{after, check, error, handlers::invoker},
    },
};

fn register_commands() -> Vec<Command> {
    vec![help::help(), privacy::privacy()]
}

pub async fn start_listener() -> Result<(), Error> {
    let settings = appsettings::load()?;

    let token = env::get("TOKEN_BOT")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: register_commands(),
            event_handler: |ctx, event, _, user_data| {
                Box::pin(invoker::listener_events_handler(ctx, event, user_data))
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

                DependencyContainer::build(settings, ready.user.id).await
            })
        });

    framework.run().await?;

    Ok(())
}