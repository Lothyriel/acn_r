use dotenv::dotenv;
use serenity::{framework::standard::StandardFramework, prelude::GatewayIntents, Client};
use std::env;

#[path = "config/group_registry.rs"]
mod group_registry;
use group_registry::FrameworkExtensions;

#[path = "config/event_handler.rs"]
mod event_handler;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("TOKEN_BOT").expect("Discord Token não encontrado vei...");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(";").with_whitespace(true))
        .register_groups();

    let mut client = Client::builder(&token, GatewayIntents::all())
        .framework(framework)
        .event_handler(event_handler::AcnHandler)
        .await
        .expect("Erro fatal");

    if let Err(error) = client.start().await {
        println!("Client error: {:?}", error);
    }
}
