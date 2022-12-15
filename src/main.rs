use dotenv::dotenv;
use serenity::{
    framework::standard::StandardFramework, prelude::GatewayIntents, Client,
};
use std::env;

#[path ="config/registerGroups.rs"]
mod frameworkExtensions;
use frameworkExtensions::FrameworkExtensions;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("TOKEN_BOT").expect("Discord Token não encontrado vei...");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(";").with_whitespace(true))
        .registerGroups();

    let mut client = Client::builder(&token, GatewayIntents::all())
        .framework(framework)
        .event_handler(misc::Misc::default())
        .await
        .expect("Erro fatal");

    if let Err(error) = client.start().await {
        println!("Client error: {:?}", error);
    }
}