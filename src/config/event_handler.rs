use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        prelude::{InviteCreateEvent, TypingStartEvent},
        voice::VoiceState,
    },
    prelude::{Context, EventHandler},
};
pub struct AcnHandler;

#[async_trait]
impl EventHandler for AcnHandler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Estamos totalmente dentro! {}", ready.user.name);
    }

    async fn invite_create(&self, _ctx: Context, event: InviteCreateEvent) {
        println!("Estão chamando randoms.... no {}", event.channel_id)
    }

    async fn typing_start(&self, _ctx: Context, _: TypingStartEvent) {        
        println!("Estou pressentindo merda...")
    }

    async fn voice_state_update(&self, _ctx: Context, state: VoiceState) {
        let member = state.member.expect("Usuário Válido");

        let action = if state.self_mute { "Mutou" } else { "Desmutou" };

        println!("{} {}", member.user.name, action)
    }
}