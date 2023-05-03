use std::fmt::format;

use anyhow::anyhow;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

use crate::application::models::appsettings::AppConfigurations;

#[command]
#[owners_only]
#[description("Liga/Desliga o modo debug")]
async fn debug(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let mut data = ctx.data.write().await;

    let configurations = data.get_mut::<AppConfigurations>().ok_or_else(|| {
        anyhow!(
            "Não tem {} cadastrado vei...",
            stringify!($AppConfigurations)
        )
    })?;

    configurations.debug = !configurations.debug;

    let option = if configurations.debug {
        "Ligado"
    } else {
        "Desligado"
    };

    msg.reply(&ctx.http, format!("O modo debug está {option}"))
        .await?;

    Ok(())
}
