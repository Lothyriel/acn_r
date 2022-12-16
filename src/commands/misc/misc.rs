use serenity::{
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::prelude::Message,
    prelude::Context,
};

#[group]
#[commands(att)]
struct Misc;

#[command]
async fn att(ctx: &Context, msg: &Message) -> CommandResult {
    let text = "Seu pau é infelizmente muito pequeno para utilizar este comando";

    msg.reply(ctx, text).await?;

    Ok(())
}