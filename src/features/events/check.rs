use anyhow::Result;

use crate::extensions::serenity::Context;

pub async fn handler(ctx: Context<'_>) -> Result<bool> {
    let owners = &ctx.data().services.allowed_ids;
    let owners_only = ctx.command().custom_data.downcast_ref::<bool>();

    let has_permission = match owners_only {
        Some(_) => owners.contains(&ctx.author().id.get()),
        None => true,
    };

    Ok(has_permission)
}
