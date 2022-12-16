use dotenv::dotenv;
use serenity::{framework::standard::StandardFramework, prelude::GatewayIntents, Client};
use std::env;

mod commands;
mod config;
mod data;
mod utils;
use crate::config::{event_handler, group_registry::FrameworkExtensions};

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
