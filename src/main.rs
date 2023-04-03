use dotenv::dotenv;
use log::error;
use serenity::{framework::standard::StandardFramework, prelude::GatewayIntents, Client};
use std::env;

use extensions::group_registry::{DependenciesExtensions, FrameworkExtensions};
use features::{buckets::eh_mito, events::invoker::Handler};

mod application;
mod extensions;
mod features;

#[tokio::main]
async fn main() {
    match dotenv() {
        Ok(_) => (),
        Err(e) => error!("Não consegui carregar o .env: {}", e),
    };

    env_logger::init();
    let token = env::var("TOKEN_BOT").expect("Discord Token não encontrado vei...");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!").with_whitespace(true))
        .register_groups()
        .bucket("pirocudo", |b| b.check(|c, m| Box::pin(eh_mito(c, m))))
        .await;

    let mut client = Client::builder(&token, GatewayIntents::all())
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Erro fatal");

    client.register_dependencies().await;

    if let Err(error) = client.start().await {
        println!("Client error: {:?}", error);
    }
}
