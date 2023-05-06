use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

#[command]
async fn random(_ctx: &Context, _msg: &Message, _args: Args) -> CommandResult {
    let _now = chrono::Utc::now();

    //let r34_services = ctx.get_dependency::<>().await?;

    let _message = _args.rest();

    Ok(())
}
