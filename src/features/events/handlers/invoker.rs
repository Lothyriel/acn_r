use anyhow::Result;
use poise::serenity_prelude::{Context, Event};

use crate::{
    application::dependency_configuration::DependencyContainer,
    features::events::handlers::{
        guild_updated, invite_created, member_added, member_removed, member_updated, message,
        ready, voice_updated,
    },
};

pub async fn songbird_handler(
    ctx: &Context,
    event: &Event<'_>,
    data: &DependencyContainer,
) -> Result<()> {
    match event {
        Event::Message { new_message } => message::handler(ctx, data, new_message).await,
        Event::Ready { data_about_bot } => {
            let message = format!(
                "Estamos totalmente dentro! {} como acn_r",
                data_about_bot.user.name
            );
            ready::handler(ctx, data, message).await
        }
        Event::InviteCreate { data } => invite_created::handler(ctx, data).await,
        Event::VoiceStateUpdate { old, new } => voice_updated::handler(ctx, old, new, data).await,
        Event::GuildMemberAddition { new_member } => member_added::handler(ctx, new_member).await,
        Event::GuildMemberRemoval {
            guild_id,
            user,
            member_data_if_available,
        } => member_removed::handler(ctx, guild_id, user, member_data_if_available).await,
        Event::GuildMemberUpdate {
            old_if_available: _,
            new,
        } => member_updated::handler(new, data).await,
        Event::GuildUpdate {
            old_data_if_available: _,
            new_but_incomplete,
        } => guild_updated::handler(new_but_incomplete, data).await,
        _ => Ok(()),
    }
}
