use dotenv::dotenv;
use serenity::{framework::standard::StandardFramework, prelude::GatewayIntents, Client};
use std::env;

#[path = "./commands/general.rs"]
mod general;

mod bot;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("TOKEN_BOT").expect("Discord Token não encontrado vei...");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(";"))
        .group(&general::GENERAL_GROUP);

    let intents = GatewayIntents::all();

    let mut client = Client::builder(&token, intents)
        .event_handler(bot::Bot::default())
        .framework(framework)
        .await
        .expect("Erro fatal");

    if let Err(error) = client.start().await {
        println!("Client error: {:?}", error);
    }
}
