use anyhow::Error;
use env_logger::Target;
use serenity::{model::prelude::UserId, prelude::GatewayIntents};
use std::collections::HashSet;

use application::{
    infra::env_var,
    services::{appsettings_service, dependency_configuration::DependencyContainer},
};
use features::{
    commands::groups_configuration,
    events::{after, error, invoker},
};

pub mod application;
pub mod extensions;
pub mod features;

pub async fn start_application() -> Result<(), Error> {
    env_logger::builder().target(Target::Stdout).try_init()?;
    dotenv::dotenv().ok();

    let token = env_var::get("TOKEN_BOT")?;

    let settings = appsettings_service::load()?;
    let _owners: HashSet<_> = settings.allowed_ids.iter().map(|i| UserId(*i)).collect();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: groups_configuration::register_commands(),
            event_handler: |ctx, event, frame, user_data| {
                Box::pin(invoker::handler(ctx, event, frame, user_data))
            },
            on_error: |error| Box::pin(error::handler(error)),
            post_command: |ctx| Box::pin(after::handler(ctx)),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(String::from("!")),
                mention_as_prefix: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .token(token)
        .intents(GatewayIntents::all())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(DependencyContainer::build(settings).await?)
            })
        });

    framework.run().await?;

    Ok(())
}
