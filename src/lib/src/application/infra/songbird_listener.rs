use anyhow::{anyhow, Error};
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use lavalink_rs::async_trait;
use log::warn;
use mongodb::bson::{spec::BinarySubtype, Binary};
use poise::serenity_prelude::Http;
use songbird::{
    events::context_data::{SpeakingUpdateData, VoiceData},
    model::{
        id::UserId,
        payload::{ClientDisconnect, Speaking},
    },
    Event, EventContext, EventHandler,
};
use std::sync::Arc;

use crate::{
    application::{models::entities::voice::VoiceSnippet, repositories::voice::VoiceRepository},
    extensions::log_ext::LogExt,
};

struct Snippet {
    bytes: Vec<u8>,
    date: DateTime<Utc>,
    mapping: Option<UserId>,
}

pub struct VoiceController {
    accumulator: DashMap<u32, Snippet>,
    repository: VoiceRepository,
    http: Arc<Http>,
}

const BUFFER_LIMIT: usize = 100_000;

impl VoiceController {
    pub fn new(repository: VoiceRepository, http: Arc<Http>) -> Self {
        Self {
            accumulator: DashMap::new(),
            http,
            repository,
        }
    }

    async fn handle_speaking_update(
        &self,
        data: &SpeakingUpdateData,
        guild_id: u64,
    ) -> Result<(), Error> {
        if data.speaking {
            return Ok(());
        }

        let (buffer_size, maybe_id) = {
            let maybe_snippet = self.accumulator.get(&data.ssrc);

            match maybe_snippet {
                Some(s) => (s.bytes.len(), s.mapping),
                None => return Ok(()),
            }
        };

        if buffer_size >= BUFFER_LIMIT {
            let id = maybe_id.ok_or_else(|| anyhow!("Buffer overflow without Mapped Id"))?;
            self.flush(data.ssrc, id, guild_id).await?;
        }

        Ok(())
    }

    fn handle_speaking_state_update(&self, data: &Speaking) -> Result<(), Error> {
        match data.user_id {
            Some(_) => {
                match self.accumulator.get_mut(&data.ssrc) {
                    Some(mut snippet) => snippet.mapping = data.user_id,
                    None => {
                        self.accumulator.insert(
                            data.ssrc,
                            Snippet {
                                bytes: vec![],
                                date: chrono::Utc::now(),
                                mapping: data.user_id,
                            },
                        );
                    }
                };
            }
            None => {
                warn!("Isso acontece muito?")
            }
        }

        Ok(())
    }

    async fn handle_client_disconnect(
        &self,
        data: &ClientDisconnect,
        guild_id: u64,
    ) -> Result<(), Error> {
        let ssrc = self
            .accumulator
            .iter()
            .find(|a| a.mapping == Some(data.user_id))
            .ok_or_else(|| {
                anyhow!("Client disconnected without sending a SpeakingStateUpdate event")
            })?;

        self.flush(*ssrc.key(), data.user_id, guild_id).await
    }

    fn handle_voice_packet(&self, data: &VoiceData<'_>) -> Result<(), Error> {
        let key = data.packet.ssrc;
        let mut bytes = data.packet.payload[data.payload_offset..].to_owned();

        match self.accumulator.get_mut(&key) {
            Some(mut m) => m.bytes.append(&mut bytes),
            None => {
                let snippet = Snippet {
                    bytes,
                    date: chrono::Utc::now(),
                    mapping: None,
                };

                self.accumulator.insert(key, snippet);
            }
        }

        Ok(())
    }

    async fn flush(&self, key: u32, user_id: UserId, guild_id: u64) -> Result<(), Error> {
        let user = self.http.get_user(user_id.0).await?;

        let (bytes, date) = {
            let mut snippet = match self.accumulator.get_mut(&key) {
                Some(r) => r,
                None => {
                    warn!("Usuário {user_id} desconectou sem nunca falar nada");
                    return Ok(());
                }
            };

            if user.bot {
                snippet.bytes.clear();
                return Ok(());
            }

            let bytes = snippet.bytes.to_owned();
            let date = snippet.date;

            snippet.date = chrono::Utc::now();
            snippet.bytes.clear();

            (bytes, date)
        };

        let snippet = VoiceSnippet {
            bytes: Binary {
                subtype: BinarySubtype::Generic,
                bytes,
            },
            date,
            user_id: user_id.0,
            guild_id,
        };

        self.repository.add_voice_snippet(snippet).await
    }
}

pub struct Receiver {
    controller: Arc<VoiceController>,
    guild_id: u64,
}

impl Receiver {
    pub fn new(controller: Arc<VoiceController>, guild_id: u64) -> Self {
        Self {
            controller,
            guild_id,
        }
    }
}

#[async_trait]
impl EventHandler for Receiver {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        handler(self.guild_id, ctx, self.controller.to_owned())
            .await
            .log();

        None
    }
}

async fn handler(
    guild_id: u64,
    ctx: &EventContext<'_>,
    controller: Arc<VoiceController>,
) -> Result<(), Error> {
    match ctx {
        EventContext::SpeakingStateUpdate(data) => controller.handle_speaking_state_update(data),
        EventContext::VoicePacket(data) => controller.handle_voice_packet(data),
        EventContext::SpeakingUpdate(data) => {
            controller.handle_speaking_update(data, guild_id).await
        }
        EventContext::ClientDisconnect(disconnect) => {
            controller
                .handle_client_disconnect(disconnect, guild_id)
                .await
        }
        _ => Err(anyhow!(
            "This handler shoudn't be subscribed to other events"
        )),
    }
}
