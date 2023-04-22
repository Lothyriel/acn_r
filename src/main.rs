use anyhow::{anyhow, Error};
use serenity::{
    framework::standard::StandardFramework, model::prelude::UserId, prelude::GatewayIntents, Client,
};

use application::{infra::env_var, services::appsettings};
use extensions::{
    group_registry::{DependenciesExtensions, FrameworkExtensions},
    log_ext::LogExt,
};
use features::{
    commands::help,
    events::{after, invoker},
};

mod application;
mod extensions;
mod features;

#[tokio::main]
async fn main() {
    start_application().await.log()
}

async fn start_application() -> Result<(), Error> {
    dotenv::dotenv().map_err(|e| anyhow!("Não consegui carregar o .env: {}", e))?;

    env_logger::init();
    let token = env_var::get("TOKEN_BOT")?;

    let settings = appsettings::load()?;
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
