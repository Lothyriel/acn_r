use std::sync::Arc;

use anyhow::{anyhow, Error};
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use lavalink_rs::async_trait;
use songbird::{Event, EventContext, EventHandler};

use crate::{application::repositories::voice::VoiceRepository, extensions::log_ext::LogExt};

struct Snippet {
    bytes: Vec<u8>,
    date: DateTime<Utc>,
}

pub struct VoiceController {
    accumulator: DashMap<u64, Snippet>,
    repository: VoiceRepository,
}

impl VoiceController {
    pub fn new(repository: VoiceRepository) -> Self {
        Self {
            accumulator: DashMap::new(),
            repository,
        }
    }
}

pub struct Receiver {
    controller: Arc<VoiceController>,
}

impl Receiver {
    pub fn new(controller: Arc<VoiceController>) -> Self {
        Self { controller }
    }
}

#[async_trait]
impl EventHandler for Receiver {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        handler(ctx).await.log();

        None
    }
}

async fn handler(ctx: &EventContext<'_>) -> Result<(), Error> {
    match ctx {
        // Speaking state update, typically describing how another voice
        // user is transmitting audio data. Clients must send at least one such
        // packet to allow SSRC/UserID matching.
        EventContext::SpeakingStateUpdate(data) => {
            println!(
                "Speaking state update: user {:?} has SSRC {:?}, using {:?}",
                data.user_id, data.ssrc, data.speaking,
            );
        }
        // Speaking state transition, describing whether a given source has started/stopped
        // transmitting. This fires in response to a silent burst, or the first packet
        // breaking such a burst.
        EventContext::SpeakingUpdate(data) => {
            println!(
                "Source {} has {} speaking.",
                data.ssrc,
                if data.speaking { "started" } else { "stopped" },
            );
        }
        EventContext::VoicePacket(data) => {
            let audio = data
                .audio
                .as_ref()
                .ok_or_else(|| anyhow!("Could not decode packet"))?;

            println!(
                "Audio packet sequence {:05} has {:04} bytes (decompressed from {}), SSRC {}",
                data.packet.sequence.0,
                audio.len() * std::mem::size_of::<i16>(),
                data.packet.payload.len(),
                data.packet.ssrc,
            );
        }
        EventContext::ClientDisconnect(disconnect) => {
            println!("Client disconnected: user {:?}", disconnect.user_id)
        }
        _ => {
            return Err(anyhow!(
                "This handler shoudn't be subscribed to other events"
            ))
        }
    }

    Ok(())
}
