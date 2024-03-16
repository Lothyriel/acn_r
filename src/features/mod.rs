use anyhow::Error;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use songbird::SerenityInit;

use crate::{
    application::{
        dependency_configuration::DependencyContainer,
        infra::{appsettings::AppSettings, env},
    },
    extensions::serenity::{context_ext::get_songbird_client, Command},
    features::commands::*,
};

use self::events::*;

mod commands;
mod events;

fn register_commands(groups: Vec<Vec<Command>>) -> Vec<Command> {
    let mut commands = vec![help::help()];

    for mut command in groups {
        commands.append(command.as_mut());
    }

    commands
}

fn register_groups() -> Vec<Vec<Command>> {
    vec![r34::group(), misc::group(), jukebox::group()]
}

pub async fn start() -> Result<(), Error> {
    let settings = AppSettings::load()?;
    let token = env::get("TOKEN_BOT")?;

    ClientBuilder::new(token, GatewayIntents::all())
        .register_songbird()
        .framework(get_framework(&settings))
        .await?
        .start()
}

fn get_framework(settings: &AppSettings) -> poise::FrameworkBuilder<DependencyContainer, Error> {
    poise::Framework::builder()
        .options(get_options(settings))
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                let songbird = get_songbird_client(ctx).await?;

                DependencyContainer::build(settings, songbird, ready.user.id).await
            })
        })
}

fn get_options(settings: &AppSettings) -> poise::FrameworkOptions<DependencyContainer, Error> {
    poise::FrameworkOptions {
        event_handler: |ctx, event, _, user_data| {
            Box::pin(handlers::invoker::songbird_handler(ctx, event, user_data))
        },
        commands: register_commands(register_groups()),
        on_error: |error| Box::pin(error::handler(error)),
        post_command: |ctx| Box::pin(after::handler(ctx)),
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(settings.prefix.to_owned()),
            mention_as_prefix: true,
            ..Default::default()
        },
        command_check: Some(|ctx| Box::pin(check::handler(ctx))),
        ..Default::default()
    }
}
