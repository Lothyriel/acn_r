use anyhow::Error;
use futures::future::join_all;
use log::warn;
use poise::serenity_prelude::Context;

use crate::{
    application::dependency_configuration::DependencyContainer, extensions::log_ext::LogErrorsExt,
};

pub async fn handler(
    ctx: &Context,
    container: &DependencyContainer,
    message: String,
) -> Result<(), Error> {
    let allowed_ids = &container.services.allowed_ids;

    warn!("{message}");

    let tasks = allowed_ids
        .iter()
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
