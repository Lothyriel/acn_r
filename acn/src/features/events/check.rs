use anyhow::Error;

use crate::application::Context;

pub async fn handler(ctx: Context<'_>) -> Result<bool, Error> {
    let owners = &ctx.data().services.allowed_ids;
    let owners_only = ctx.command().custom_data.downcast_ref::<bool>();

    let has_permission = match owners_only {
        Some(_) => owners.contains(&ctx.author().id.0),
        None => true,
    };

    Ok(has_permission)
}