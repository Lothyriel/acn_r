use anyhow::{anyhow, Error};
use dotenv::dotenv;
use serenity::{framework::standard::StandardFramework, prelude::GatewayIntents, Client};
use std::env;

use extensions::{
    group_registry::{DependenciesExtensions, FrameworkExtensions},
    log_ext::LogExt,
};
use features::{events::invoker::Handler};

mod application;
mod extensions;
mod features;

#[tokio::main]
async fn main() {
    start_application().await.log()
}

async fn start_application() -> Result<(), Error> {
    dotenv().map_err(|e| anyhow!("Não consegui carregar o .env: {}", e))?;

    env_logger::init();
    let token = get_token_bot()?;

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!").with_whitespace(true))
        .register_groups();

    let mut client = Client::builder(&token, GatewayIntents::all())
        .framework(framework)
        .event_handler(Handler)
        .await?;

    client.register_dependencies().await?;

    client.start().await?;

    Ok(())
}

pub fn get_token_bot() -> Result<String, Error> {
    env::var("TOKEN_BOT").map_err(|_| anyhow!("TOKEN_BOT não definido nas variáveis de ambiente"))
}
