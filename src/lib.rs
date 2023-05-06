use anyhow::Error;
use serenity::{
    framework::standard::StandardFramework, model::prelude::UserId, prelude::GatewayIntents, Client,
};

use application::{infra::env_var, services::appsettings_service};
use extensions::group_registry::{DependenciesExtensions, FrameworkExtensions};
use features::{
    commands::help,
    events::{after, invoker},
};

pub mod application;
pub mod extensions;
pub mod features;

pub async fn start_application() -> Result<(), Error> {
    env_logger::init();
    dotenv::dotenv().ok();

    let token = env_var::get("TOKEN_BOT")?;

    let settings = appsettings_service::load()?;
    let owners = settings.allowed_ids.iter().map(|i| UserId(*i)).collect();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!").with_whitespace(true).owners(owners))
        .register_groups()
        .help(&help::HELP)
        .after(after::after);

    let mut client = Client::builder(&token, GatewayIntents::all())
        .framework(framework)
        .event_handler(invoker::Handler)
        .await?;

    client.register_dependencies(settings).await?;

    client.start().await?;

    Ok(())
}
