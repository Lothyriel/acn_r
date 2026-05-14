use anyhow::Result;
use futures::future::join_all;
use poise::serenity_prelude::{Context, CreateMessage, UserId};

use crate::{
    application::dependency_configuration::DependencyContainer,
    extensions::std_ext::collapse_errors,
};

pub async fn handler(
    _ctx: &Context,
    _container: &DependencyContainer,
    username: &str,
) -> Result<()> {
    let message = format!("Estamos totalmente dentro! {}", username);

    log::info!("{}", message);

    Ok(())
}

#[allow(dead_code)]
async fn dm_greetings(ctx: &Context, container: &DependencyContainer, message: &str) -> Result<()> {
    let allowed_ids = &container.allowed_ids;

    let tasks = allowed_ids.iter().map(|u| send_greetings(ctx, *u, message));

    let tasks_results = join_all(tasks).await;

    _ = collapse_errors(tasks_results.into_iter())?;

    Ok(())
}

async fn send_greetings(ctx: &Context, id: UserId, message: &str) -> Result<()> {
    let user = ctx.http.get_user(id).await?;

    let channel = user.create_dm_channel(&ctx.http).await?;

    channel
        .send_message(&ctx.http, CreateMessage::new().content(message))
        .await?;

    Ok(())
}
