use anyhow::{anyhow, Error};
use dotenv::dotenv;
use log::error;
use serenity::{framework::standard::StandardFramework, prelude::GatewayIntents, Client};
use std::env;

use extensions::{
    group_registry::{DependenciesExtensions, FrameworkExtensions},
    log_ext::LogExt,
};
use features::{buckets::eh_mito, events::invoker::Handler};

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
    let token = env::var("TOKEN_BOT")
        .map_err(|_| anyhow!("TOKEN_BOT não definido nas variáveis de ambiente"))?;

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!").with_whitespace(true))
        .register_groups()
        .bucket("pirocudo", |b| b.check(|c, m| Box::pin(eh_mito(c, m))))
        .await;

    let mut client = Client::builder(&token, GatewayIntents::all())
        .framework(framework)
        .event_handler(Handler)
        .await?;

    client.register_dependencies().await;
    client.start();

    Ok(())
}
