use dotenv::dotenv;
use serenity::{
    async_trait,
    framework::standard::{
        macros::{command, group},
        CommandResult, StandardFramework,
    },
    model::prelude::{command, Message, Ready},
    prelude::*,
    prelude::{Context, EventHandler, GatewayIntents},
    Client,
};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("TOKEN_BOT").expect("Discord Token não encontrado vei...");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token, GatewayIntents::GUILD_MEMBERS)
        .framework(framework)
        .await
        .expect("Erro fatal");

    if let Err(error) = client.start().await {
        println!("Client error: {:?}", error);
    }
}
#[group]
#[commands(ping)]
struct General;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

struct Sexo;

#[async_trait]
impl EventHandler for Sexo {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!hello" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
                //error!("Error sending message: {:?}", e);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        print!("{} is connected!", ready.user.name);
    }
}
