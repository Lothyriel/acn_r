use anyhow::Error;
use serenity::prelude::GatewayIntents;

use application::{
    dependency_configuration::DependencyContainer,
    infra::{appsettings, env},
};
use features::{
    commands::groups_configuration,
    events::{after, check, error, invoker},
};

pub mod application;
pub mod extensions;
pub mod features;

pub async fn start_application() -> Result<(), Error> {
    env::init()?;

    let token = env::get("TOKEN_BOT")?;
    let settings = appsettings::load()?;
    let prefix = settings.prefix.to_owned();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: groups_configuration::register_commands(),
            event_handler: |ctx, event, _, user_data| {
                Box::pin(invoker::handler(ctx, event, user_data))
            },
            on_error: |error| Box::pin(error::handler(error)),
            post_command: |ctx| Box::pin(after::handler(ctx)),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(prefix),
                mention_as_prefix: true,
                ..Default::default()
            },
            command_check: Some(|ctx| Box::pin(check::handler(ctx))),
            ..Default::default()
        })
        .token(token)
        .intents(GatewayIntents::all())
        .setup(|ctx, _, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(DependencyContainer::build(settings).await?)
            })
        });

    framework.run().await?;

    Ok(())
}
