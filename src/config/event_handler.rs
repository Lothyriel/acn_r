use serenity::{
    async_trait,
    model::gateway::Ready,
    prelude::{Context, EventHandler},
};
pub struct AcnHandler;

#[async_trait]
impl EventHandler for AcnHandler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Estamos totalmente dentro! {}", ready.user.name);
    }
}
