use anyhow::Error;
use poise::serenity_prelude::GatewayIntents;
use songbird::{Config, SerenityInit};

use crate::{
    application::{
        dependency_configuration::DependencyContainer,
        infra::{appsettings, env},
    },
    extensions::serenity::Command,
    features::{
        commands::{help, jukebox, misc, r34, reactions},
        events::{after, check, error, handlers::invoker},
    },
};

fn register_groups() -> Vec<Vec<Command>> {
    vec![
        r34::group(),
        misc::group(),
        jukebox::group(),
        reactions::group(),
    ]
}

fn register_commands() -> Vec<Command> {
    let mut commands = vec![help::help()];

    for mut command in register_groups() {
        commands.append(command.as_mut());
    }

    commands
}

pub async fn start_acn() -> Result<(), Error> {
    let settings = appsettings::load()?;
    let token = env::get("TOKEN_BOT")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            event_handler: |ctx, event, _, user_data| {
                Box::pin(invoker::songbird_handler(ctx, event, user_data))
            },
            commands: register_commands(),
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

                DependencyContainer::build(settings, ready.user.id).await
            })
        });

    framework.run().await?;

    Ok(())
}