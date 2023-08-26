use anyhow::{anyhow, Error};
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use futures::future::join_all;
use hound::{WavSpec, WavWriter};
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
use std::{io::Cursor, sync::Arc};
use symphonia::{
    core::{
        audio::Layout,
        codecs::{CodecParameters, CODEC_TYPE_PCM_S16LE},
        io::MediaSourceStream,
        sample::SampleFormat,
    },
    default::{self},
};

use crate::{
    application::{models::entities::voice::VoiceSnippet, repositories::voice::VoiceRepository},
    extensions::{log_ext::LogExt, serenity::Context, std_ext::VecResultErrorExt},
};

struct Snippet {
    bytes: Vec<i16>,
    date: DateTime<Utc>,
    mapping: Option<UserInfo>,
}

#[derive(PartialEq, Copy, Clone)]
struct UserInfo {
    user_id: u64,
    guild_id: u64,
}

pub struct VoiceController {
    accumulator: DashMap<u32, Snippet>,
    repository: VoiceRepository,
    http: Arc<Http>,
}

const BUFFER_LIMIT: usize = 1_000_000;

impl VoiceController {
    pub fn new(repository: VoiceRepository, http: Arc<Http>) -> Self {
        Self {
            accumulator: DashMap::new(),
            http,
            repository,
        }
    }

    pub async fn flush_all(&self) -> Result<(), Error> {
        let flush_tasks = self
            .accumulator
            .iter()
            .flat_map(|s| s.mapping.map(|id| self.flush(*s.key(), id.user_id)));

        join_all(flush_tasks).await.all_successes()?;

        self.accumulator.clear();

        Ok(())
    }

    async fn handle_speaking_update(&self, data: &SpeakingUpdateData) -> Result<(), Error> {
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
            self.flush(data.ssrc, id.user_id).await?;
        }

        Ok(())
    }

    fn handle_speaking_state_update(&self, data: &Speaking, guild_id: u64) -> Result<(), Error> {
        if data.user_id.is_none() {
            return Ok(());
        }

        let get_info = |i: UserId| UserInfo {
            user_id: i.0,
            guild_id,
        };

        match self.accumulator.get_mut(&data.ssrc) {
            Some(mut snippet) => snippet.mapping = data.user_id.map(get_info),
            None => {
                self.accumulator.insert(
                    data.ssrc,
                    Snippet {
                        bytes: vec![],
                        date: chrono::Utc::now(),
                        mapping: data.user_id.map(get_info),
                    },
                );
            }
        };

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
            .find(|a| {
                a.mapping
                    == Some(UserInfo {
                        user_id: data.user_id.0,
                        guild_id,
                    })
            })
            .ok_or_else(|| {
                anyhow!("Client disconnected without sending a SpeakingStateUpdate event")
            })?;

        self.flush(*ssrc.key(), data.user_id.0).await
    }

    fn handle_voice_packet(&self, data: &VoiceData<'_>) -> Result<(), Error> {
        let key = data.packet.ssrc;

        let mut bytes = data
            .audio
            .to_owned()
            .ok_or_else(|| anyhow!("Decoded audio was not present"))?;

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

    async fn flush(&self, key: u32, user_id: u64) -> Result<(), Error> {
        let user = self.http.get_user(user_id).await?;

        let (bytes, date, guild_id) = {
            let mut snippet = match self.accumulator.get_mut(&key) {
                Some(r) => r,
                None => {
                    warn!("Usuário {} desconectou sem nunca falar nada", user_id);
                    return Ok(());
                }
            };

            if user.bot {
                snippet.bytes.clear();
                return Ok(());
            }

            let guild_id = match &snippet.mapping {
                Some(i) => i.guild_id,
                None => return Ok(()),
            };

            let bytes = snippet.bytes.to_owned();
            let date = snippet.date;

            snippet.date = chrono::Utc::now();
            snippet.bytes.clear();

            (bytes, date, guild_id)
        };

        let mut buffer = vec![];
        to_wav(bytes.as_slice(), &mut buffer)?;

        // let mp3 = to_mp3(buffer);

        // OpenOptions::new()
        //     .append(true)
        //     .create(true)
        //     .open(format!("audio_{}.mp3", user.name))
        //     .unwrap()
        //     .write_all(mp3.as_slice())
        //     .unwrap();

        let snippet = VoiceSnippet {
            voice_data: Binary {
                subtype: BinarySubtype::Generic,
                bytes: buffer,
            },
            date,
            user_id,
            guild_id,
        };

        self.repository.add_voice_snippet(snippet).await
    }
}

fn to_wav(pcm_samples: &[i16], buffer: &mut Vec<u8>) -> Result<(), Error> {
    let spec = WavSpec {
        channels: 2,
        sample_rate: 48000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let cursor = Cursor::new(buffer);

    let mut writer = WavWriter::new(cursor, spec)?;

    for &sample in pcm_samples {
        writer.write_sample(sample)?;
    }

    Ok(())
}

#[allow(dead_code)]
fn to_mp3(buffer: Vec<u8>) -> Result<Vec<u8>, Error> {
    let _codec_parameters = CodecParameters {
        codec: CODEC_TYPE_PCM_S16LE,
        sample_rate: Some(48_000),
        sample_format: Some(SampleFormat::U16),
        bits_per_coded_sample: Some(16),
        channel_layout: Some(Layout::Stereo),
        ..Default::default()
    };

    let codec_registry = default::get_codecs();

    let probe = default::get_probe();

    let mss = MediaSourceStream::new(Box::new(Cursor::new(buffer)), Default::default());

    let mut reader = probe
        .format(
            &Default::default(),
            mss,
            &Default::default(),
            &Default::default(),
        )?
        .format;

    let track = reader
        .tracks()
        .first()
        .ok_or_else(|| anyhow!("No tracks found"))?;

    let mut decoder = codec_registry.make(&track.codec_params, &Default::default())?;

    let packet = reader.next_packet()?;
    let _audio_buffer = decoder.decode(&packet)?;

    //let a: SampleFormat = audio_buffer.into();

    //let a: AudioBuffer<u32> = audio_buffer.make_equivalent();

    todo!()
}

pub struct Receiver {
    controller: Arc<VoiceController>,
    guild_id: u64,
}

impl Receiver {
    pub fn from_ctx(ctx: &Context<'_>, guild_id: u64) -> Self {
        Self {
            controller: ctx.data().services.voice_controller.to_owned(),
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
        EventContext::SpeakingStateUpdate(data) => {
            controller.handle_speaking_state_update(data, guild_id)
        }
        EventContext::VoicePacket(data) => controller.handle_voice_packet(data),
        EventContext::SpeakingUpdate(data) => controller.handle_speaking_update(data).await,
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
