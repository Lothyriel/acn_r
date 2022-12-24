use anyhow::Error;
use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        prelude::{InviteCreateEvent, Member},
    },
    prelude::{Context, EventHandler, Mentionable},
};

// use crate::utils::{guild_ext::GuildExt, log::FutureExt};
pub struct AcnHandler;

#[async_trait]
impl EventHandler for AcnHandler {
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        handle_guild_member_addition(ctx, new_member).await;
    }

    async fn invite_create(&self, ctx: Context, event: InviteCreateEvent) {
        handle_invite_create(ctx, event).await;
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("Estamos totalmente dentro! {}", ready.user.name);
    }
}

async fn handle_invite_create(ctx: Context, event: InviteCreateEvent) -> Result<(), Error> {
    let channel = event.channel_id;
    let inviter = event.inviter?;

    let response = format!("{} Está chamando randoms....", inviter.mention());
    channel.say(&ctx.http, response).await?;

    Ok(())
}

async fn handle_guild_member_addition(ctx: Context, new_member: Member) -> Result<(), Error> {
    let response = format!("Novo random detectado: {}", new_member.mention());

    new_member
        .guild_id
        .say_on_main_text_channel(&ctx.http, &response)
        .await?;

    Ok(())
}
