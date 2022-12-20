use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        prelude::{InviteCreateEvent, Member},
    },
    prelude::{Context, EventHandler, Mentionable},
    Error,
};

use crate::utils::log::log;
pub struct AcnHandler;

#[async_trait]
impl EventHandler for AcnHandler {
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        log(handle_guild_member_addition(ctx, new_member)).await;
    }

    async fn invite_create(&self, ctx: Context, event: InviteCreateEvent) {
        log(handle_invite_create(ctx, event)).await;
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("Estamos totalmente dentro! {}", ready.user.name);
    }
}

async fn handle_invite_create(ctx: Context, event: InviteCreateEvent) -> Result<(), Error> {
    let channel = event.channel_id;
    let inviter = event
        .inviter
        .ok_or_else(|| Error::Other("Não achei quem criou o convite"))?;

    let response = format!("{} Está chamando randoms....", inviter.mention());
    channel.say(&ctx.http, response).await?;

    Ok(())
}

async fn handle_guild_member_addition(ctx: Context, new_member: Member) -> Result<(), Error> {
    let channels = new_member.guild_id.channels(&ctx.http).await?;

    let text_channel = channels
        .values()
        .min_by(|a, b| a.position.cmp(&b.position))
        .ok_or_else(|| Error::Other("Não achei um canal"))?;

    let response = format!("Novo random detectado: {}", new_member.mention());
    text_channel.say(&ctx.http, response).await?;

    Ok(())
}
