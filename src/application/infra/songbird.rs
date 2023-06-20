use lavalink_rs::async_trait;
use songbird::{Event, EventContext, EventHandler};

struct Receiver;

impl Receiver {
    pub fn new() -> Self {
        // You can manage state here, such as a buffer of audio packet bytes so
        // you can later store them in intervals.
        Self {}
    }
}

#[async_trait]
impl EventHandler for Receiver {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        match ctx {
            EventContext::Track(_) => todo!(),
            EventContext::SpeakingStateUpdate(_) => todo!(),
            EventContext::SpeakingUpdate(_) => todo!(),
            EventContext::VoicePacket(_) => todo!(),
            EventContext::RtcpPacket(_) => todo!(),
            EventContext::ClientDisconnect(_) => todo!(),
            _ => todo!()
        }
    }
}
