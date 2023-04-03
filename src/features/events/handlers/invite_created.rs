use anyhow::{anyhow, Error};
use serenity::{
    model::prelude::InviteCreateEvent,
    prelude::{Context, Mentionable},
};

pub async fn handler(ctx: Context, event: InviteCreateEvent) -> Result<(), Error> {
    let channel = event.channel_id;
    let inviter = event
        .inviter
        .ok_or_else(|| anyhow!("Este convite não contém criador"))?;

    let response = format!("{} Está chamando randoms....", inviter.mention());
    channel.say(&ctx.http, response).await?;

    Ok(())
}
