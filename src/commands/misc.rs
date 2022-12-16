use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::prelude::Message,
    prelude::Context,
};

#[group]
#[commands(att)]
struct Misc;

#[command]
async fn att(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let text = "Seu pau é infelizmente muito pequeno para utilizar este comando";

    // if eh_plebe(&msg.author) {
    //     msg.reply(ctx, text).await?;
    // }
    // let mensagem = args.trimmed();
    // for grupo in self.bot.guilds:
    //     await grupo.text_channels[0].send(mensagem)

    Ok(())
}

// @commands.command(help="Mandar <msg> para todos os grupos")
// async def att(self, ctx, *msg):
//     if self.bot.eh_plebe(ctx.author):
//         return await ctx.send("Seu pau é infelizmente muito pequeno para utilizar este comando")

//     mensagem = " ".join(msg)
//     for grupo in self.bot.guilds:
//         await grupo.text_channels[0].send(mensagem)
