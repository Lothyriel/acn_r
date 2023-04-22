use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        prelude::{Guild, GuildId, InviteCreateEvent, Member, PartialGuild},
        user::User,
        voice::VoiceState,
    },
    prelude::{Context, EventHandler},
};

use crate::{
    extensions::log_ext::LogExt,
    features::events::handlers::{
        guild_updated, invite_created, member_added, member_removed, ready, voice_updated,
    },
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        member_added::handler(ctx, new_member).await.log()
    }

    async fn invite_create(&self, ctx: Context, event: InviteCreateEvent) {
        invite_created::handler(ctx, event).await.log()
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        ready::handler(ctx, ready).await.log()
    }

    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        voice_updated::handler(ctx, old, new).await.log()
    }

    async fn guild_update(&self, ctx: Context, _old: Option<Guild>, new: PartialGuild) {
        guild_updated::handler(ctx, new).await.log()
    }

    async fn guild_member_removal(
        &self,
        ctx: Context,
        id: GuildId,
        user: User,
        member: Option<Member>,
    ) {
        member_removed::handler(ctx, id, user, member).await.log()
    }
}
