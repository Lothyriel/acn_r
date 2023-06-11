use anyhow::{anyhow, Error};
use poise::serenity_prelude::{Context, InviteCreateEvent, Mentionable};

pub async fn handler(ctx: &Context, event: &InviteCreateEvent) -> Result<(), Error> {
    let identifier = event
        .channel_id
        .name(ctx)
        .await
        .unwrap_or_else(|| event.channel_id.to_string());

    let inviter = event.inviter.as_ref().ok_or_else(|| {
        anyhow!(
            "Invite to channel: {} doesn't contain an inviter",
            identifier
        )
    })?;

    let response = format!("{} Está chamando randoms....", inviter.mention());
    event.channel_id.say(ctx, response).await?;

    Ok(())
}
