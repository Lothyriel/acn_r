use log::{error, info};
use serenity::{
    framework::standard::{macros::hook, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

#[hook]
pub async fn handler(_ctx: &Context, msg: &Message, command_name: &str, command_result: CommandResult) {
    let guild_id = msg.guild_id.map(|g| g.0);

    match command_result {
        Ok(()) => info!("Processed command '{}'", command_name),
        Err(why) => error!(
            "Command '{}' in guild: '{:?}' with args: '{:?}' returned error {:?}",
            command_name,
            guild_id,
            msg,
            why
        ),
    }
}
