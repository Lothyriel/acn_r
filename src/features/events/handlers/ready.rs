use anyhow::Error;
use log::warn;
use poise::serenity_prelude::Context;
use serenity::futures::future::join_all;

use crate::{
    application::dependency_configuration::DependencyContainer, extensions::log_ext::LogErrorsExt,
};

pub async fn handler(
    ctx: &Context,
    container: &DependencyContainer,
    ready: &poise::serenity_prelude::Ready,
) -> Result<(), Error> {
    let permitidos = &container.allowed_ids;
    let message = format!("Estamos totalmente dentro! {}", ready.user.name);
    warn!("{message}");
    let tasks = permitidos
        .into_iter()
        .map(|p| send_greetings(ctx, *p, &message));

    join_all(tasks).await.log_errors();

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
