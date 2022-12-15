use serenity::{async_trait, prelude::*, model::prelude::Message};

pub struct Misc {
    id_pirocudo: u64,
    id_mito: u64,
}

#[async_trait]
impl EventHandler for Misc {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}

impl Default for Misc {
    fn default() -> Self {
        Self {
            id_pirocudo: 244922703667003392,
            id_mito: 892942296566358066,
        }
    }
}
