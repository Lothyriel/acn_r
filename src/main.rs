use dotenv::dotenv;
use serenity::{framework::standard::StandardFramework, prelude::GatewayIntents, Client};
use std::env;

use crate::{
    config::{event_handler, group_registry::FrameworkExtensions},
    data::utils::eh_mito,
};

mod commands;
mod config;
mod data;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    let token = env::var("TOKEN_BOT").expect("Discord Token não encontrado vei...");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(";").with_whitespace(true))
        .register_groups()
        .bucket("pirocudo", |b| b.check(|_, m| Box::pin(eh_mito(&m.author))))
        .await;

    let mut client = Client::builder(&token, GatewayIntents::all())
        .framework(framework)
        .event_handler(event_handler::AcnHandler)
        .await
        .expect("Erro fatal");

    if let Err(error) = client.start().await {
        println!("Client error: {:?}", error);
    }
}
