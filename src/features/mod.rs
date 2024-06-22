use anyhow::{Error, Result};
use lavalink_rs::{
    client::LavalinkClient, model::events::Events, node::NodeBuilder,
    prelude::NodeDistributionStrategy,
};
use poise::serenity_prelude::{ClientBuilder, Context, GatewayIntents};
use songbird::SerenityInit;

use crate::{
    application::{
        dependency_configuration::DependencyContainer,
        infra::{appsettings::AppSettings, env},
    },
    extensions::serenity::Command,
    features::commands::*,
};

use self::events::*;

mod commands;
mod events;

fn register_commands() -> Vec<Command> {
    let mut commands = vec![help::help()];

    let groups = vec![r34::group(), misc::group(), jukebox::group()];

    for mut command in groups {
        commands.append(command.as_mut());
    }

    commands
}

pub async fn start() -> Result<()> {
    let settings = AppSettings::load()?;
    let token = env::get("TOKEN_BOT")?;

    ClientBuilder::new(token, GatewayIntents::all())
        .register_songbird()
        .framework(get_framework(settings))
        .await?
        .start()
        .await?;

    Ok(())
}

fn get_framework(settings: AppSettings) -> poise::Framework<DependencyContainer, Error> {
    poise::Framework::builder()
        .options(get_options(&settings))
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                let client = get_lavalink_client(ctx).await?;

                DependencyContainer::build(settings, client, ready.user.id).await
            })
        })
        .build()
}

async fn get_lavalink_client(ctx: &Context) -> Result<LavalinkClient> {
    let events = Events {
        track_start: None,
        ..Default::default()
    };

    let node_local = NodeBuilder {
        hostname: "192.168.3.10:2333".to_string(),
        is_ssl: false,
        events: Events::default(),
        password: env::get("LAVALINK_PASSWORD")?,
        user_id: ctx.cache.current_user().id.into(),
        session_id: None,
    };

    let client = LavalinkClient::new(
        events,
        vec![node_local],
        NodeDistributionStrategy::round_robin(),
    )
    .await;

    Ok(client)
}

fn get_options(settings: &AppSettings) -> poise::FrameworkOptions<DependencyContainer, Error> {
    poise::FrameworkOptions {
        event_handler: |ctx, event, _, user_data| {
            Box::pin(handlers::invoker::songbird_handler(ctx, event, user_data))
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
    }
}
