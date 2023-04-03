use anyhow::Error;
use serenity::{model::prelude::Ready, prelude::Context};

use crate::{
    application::models::allowed_ids::AllowedIds,
    extensions::{dependency_ext::Dependencies, log_ext::LogExt},
};

pub async fn handler(ctx: Context, ready: Ready) -> Result<(), Error> {
    let permitidos = ctx.get_dependency::<AllowedIds>().await?;
    let message = format!("Estamos totalmente dentro! {}", ready.user.name);
    println!("{}", message);

    for id in permitidos {
        send_greetings(&ctx, id, &message).await.log();
    }

    Ok(())
}

async fn send_greetings(ctx: &Context, id: u64, message: &String) -> Result<(), Error> {
    let user = ctx.http.get_user(id).await?;
    let channel = user.create_dm_channel(&ctx.http).await?;

    channel
        .send_message(&ctx.http, |m| m.content(message))
        .await?;

    Ok(())
}
