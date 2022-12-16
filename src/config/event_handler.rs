use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        prelude::{InviteCreateEvent, Member, Presence, TypingStartEvent},
        voice::VoiceState,
    },
    prelude::{Context, EventHandler, Mentionable},
    Error,
};

use crate::utils::log::LogErrors;
pub struct AcnHandler;

#[async_trait]
impl EventHandler for AcnHandler {
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        handle_guild_member_addition(ctx, new_member).await.log();
    }

    async fn invite_create(&self, ctx: Context, event: InviteCreateEvent) {
        handle_invite_create(ctx, event).await.log();
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("Estamos totalmente dentro! {}", ready.user.name);
    }

    async fn typing_start(&self, ctx: Context, event: TypingStartEvent) {
        handle_typing_start(ctx, event).await.log();
    }

    async fn voice_state_update(&self, ctx: Context, state: VoiceState) {
        handle_voice_state_update(ctx, state).await.log();
    }

    async fn presence_update(&self, ctx: Context, new_data: Presence) {
        handle_presence_update(ctx, new_data).await.log();
    }
}

async fn handle_presence_update(ctx: Context, new_data: Presence) -> Result<(), Error> {
    let user = new_data
        .user
        .avatar
        .ok_or_else(|| Error::Other("Não encontrei o usuário"))?;

    println!("{} {:?}", user, new_data.activities);

    let dm = new_data.user.id.create_dm_channel(&ctx.http).await.unwrap();

    dm.say(&ctx.http, "Vai tomar no cu").await.unwrap();

    Ok(())
}

async fn handle_typing_start(ctx: Context, event: TypingStartEvent) -> Result<(), Error> {
    let channel = event.channel_id;

    channel
        .say(&ctx.http, "Estou pressentindo merda...")
        .await?;

    Ok(())
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

async fn handle_voice_state_update(ctx: Context, state: VoiceState) -> Result<(), Error> {
    let member = state
        .member
        .ok_or_else(|| Error::Other("Não encontrei o membro"))?;

    let action = if state.self_mute {
        "mutado"
    } else {
        "desmutado"
    };

    let guild_id = state
        .guild_id
        .ok_or_else(|| Error::Other("Não encontrei a guilda"))?;

    let channels = guild_id.channels(&ctx.http).await?;

    let text_channel = channels
        .values()
        .filter(|c| c.is_text_based())
        .min_by(|a, b| a.position.cmp(&b.position))
        .ok_or_else(|| Error::Other("Não achei um canal"))?;

    let response = format!("{} está {}", member.mention(), action);
    text_channel.say(&ctx.http, response).await?;

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
