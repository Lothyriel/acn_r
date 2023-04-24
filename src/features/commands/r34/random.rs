use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

use crate::extensions::dependency_ext::Dependencies;

#[command]
async fn random(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let now = chrono::Utc::now();

    //let r34_services = ctx.get_dependency::<>().await?;

    let _message = args.rest();

    Ok(())
}
