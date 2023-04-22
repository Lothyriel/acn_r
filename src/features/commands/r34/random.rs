use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

#[command]
async fn random(ctx: &Context, _msg: &Message, args: Args) -> CommandResult {
    let message = args.rest();

    Ok(())
}
