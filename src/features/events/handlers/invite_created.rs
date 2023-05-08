use anyhow::{anyhow, Error};
use poise::serenity_prelude::Context;
use serenity::{model::prelude::InviteCreateEvent, prelude::Mentionable};

pub async fn handler(ctx: &Context, event: &InviteCreateEvent) -> Result<(), Error> {
    let inviter = event
        .inviter
        .as_ref()
        .ok_or_else(|| anyhow!("Este convite não contém criador"))?;

    let response = format!("{} Está chamando randoms....", inviter.mention());
    event.channel_id.say(ctx, response).await?;

    Ok(())
}
