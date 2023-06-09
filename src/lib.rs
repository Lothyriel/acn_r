use anyhow::Error;
use lavalink_rs::{async_trait, gateway::LavalinkEventHandler, LavalinkClient};
use serenity::{http::Http, prelude::GatewayIntents};

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

struct LavalinkHandler;

#[async_trait]
impl LavalinkEventHandler for LavalinkHandler {}

pub async fn start_application() -> Result<(), Error> {
    let settings = init_app()?;
    let token = env::get("TOKEN_BOT")?;
    let prefix = settings.prefix.to_owned();

    let bot_id = Http::new(&token).get_current_application_info().await?;

    let lava_client = LavalinkClient::builder(bot_id.id.0)
        .set_host("127.0.0.1")
        .set_password("SENHA???")
        .build(LavalinkHandler)
        .await?;

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
                DependencyContainer::build(settings, lava_client).await
            })
        });

    framework.run().await?;

    Ok(())
}

pub fn init_app() -> Result<appsettings::AppSettings, Error> {
    env::init()?;
    appsettings::load()
}
