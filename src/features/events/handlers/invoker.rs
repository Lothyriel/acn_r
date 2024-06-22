use anyhow::Result;
use poise::serenity_prelude::{Context, FullEvent};

use crate::{
    application::dependency_configuration::DependencyContainer,
    features::events::handlers::{
        guild_updated, invite_created, member_added, member_removed, member_updated, message,
        ready, voice_updated,
    },
};

pub async fn songbird_handler(
    ctx: &Context,
    event: &FullEvent,
    data: &DependencyContainer,
) -> Result<()> {
    match event {
        FullEvent::GuildAuditLogEntryCreate {
            entry: _,
            guild_id: _,
        } => {
            todo!("SALVAR ISSO AQUI NO BANCO")
        }
        FullEvent::MessageUpdate {
            old_if_available,
            new,
            event,
        } => {
            log::warn!(
                "message update {:?} {:?} {:?}",
                old_if_available,
                new,
                event
            );
            Ok(())
        }
        FullEvent::GuildMemberAddition { new_member } => {
            member_added::handler(ctx, new_member).await
        }
        FullEvent::GuildMemberRemoval {
            guild_id,
            user,
            member_data_if_available,
        } => member_removed::handler(ctx, guild_id, user, member_data_if_available).await,
        FullEvent::GuildMemberUpdate {
            old_if_available: _,
            new: _,
            event,
        } => member_updated::handler(event, data).await,
        FullEvent::GuildUpdate {
            old_data_if_available: _,
            new_data,
        } => guild_updated::handler(new_data, data).await,
        FullEvent::InviteCreate { data } => invite_created::handler(ctx, data).await,
        FullEvent::Message { new_message } => message::handler(ctx, data, new_message).await,
        FullEvent::Ready { data_about_bot } => {
            ready::handler(ctx, data, &data_about_bot.user.name).await
        }
        FullEvent::VoiceStateUpdate { old, new } => {
            voice_updated::handler(ctx, old, new, data).await
        }
        FullEvent::WebhookUpdate {
            guild_id,
            belongs_to_channel_id: _,
        } => Ok(log::error!("Webhook update {}", guild_id)),
        FullEvent::Ratelimit { data } => Ok(log::error!("Rate limited {:?}", data)),
        _ => Ok(()),
    }
}
