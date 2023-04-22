use anyhow::Error;
use log::info;
use serenity::{futures::future::join_all, model::prelude::Ready, prelude::Context};

use crate::{
    application::models::allowed_ids::AllowedIds,
    extensions::{dependency_ext::Dependencies, log_ext::LogExt},
};

pub async fn handler(ctx: Context, ready: Ready) -> Result<(), Error> {
    let permitidos = ctx.get_dependency::<AllowedIds>().await?;
    let message = format!("Estamos totalmente dentro! {}", ready.user.name);
    info!("{message}");

    let tasks: Vec<_> = permitidos
        .into_iter()
        .map(|p| send_greetings(&ctx, p, &message))
        .collect();

    join_all(tasks).await.log();

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
