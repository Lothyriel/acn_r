use serenity::{
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::prelude::{command, Message, Ready},
    prelude::*,
    prelude::{Context, EventHandler, GatewayIntents},
    Client,
};

#[command]
async fn att(ctx: &Context, msg: &Message) -> CommandResult {
    // if self.bot.eh_plebe(ctx.author){

    
    msg.reply(
        ctx,
        "Seu pau é infelizmente muito pequeno para utilizar este comando",
    )
    .await?;
    // }

    // mensagem = " ".join(msg)
    // for grupo in self.bot.guilds:
    //     grupo.text_channels[0].send(mensagem)
    Ok(())
}
