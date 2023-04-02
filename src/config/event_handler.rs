use anyhow::{anyhow, Error};
use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        prelude::{InviteCreateEvent, Member, UserId},
    },
    prelude::{Context, EventHandler, Mentionable, TypeMapKey},
};

use crate::utils::{guild_ext::GuildExt, log::LogExt};

pub struct AcnHandler;

#[async_trait]
impl EventHandler for AcnHandler {
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        handle_guild_member_addition(ctx, new_member).await.log();
    }

    async fn invite_create(&self, ctx: Context, event: InviteCreateEvent) {
        handle_invite_create(ctx, event).await.log();
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Estamos totalmente dentro! {}", ready.user.name);
        handle_ready(ctx, ready).await.log()
    }
}

async fn handle_invite_create(ctx: Context, event: InviteCreateEvent) -> Result<(), Error> {
    let channel = event.channel_id;
    let inviter = event
        .inviter
        .ok_or_else(|| anyhow!("Este convite não contém criador"))?;

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

async fn handle_ready(ctx: Context, _ready: Ready) -> Result<(), Error> {
    let permitidos = ctx.chupar_drops::<AllowedIds>();

    Ok(())
}

struct AllowedIds;

impl TypeMapKey for AllowedIds {
    type Value = Vec<UserId>;
}

#[async_trait]
trait ChuparDrops {
    async fn chupar_drops<'a, T: TypeMapKey>(
        &'a self,
    ) -> Result<&'a <T as TypeMapKey>::Value, Error>;
}

#[async_trait]
impl ChuparDrops for Context {
    async fn chupar_drops<'a, T: TypeMapKey>(
        &'a self,
    ) -> Result<&'a <T as TypeMapKey>::Value, Error> {
        self.data
            .read()
            .await
            .get::<T>()
            .ok_or_else(|| anyhow!("Não tem {} cadastrado vei...", std::any::type_name::<T>()))
    }
}
