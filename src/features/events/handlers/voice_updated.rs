use anyhow::Error;
use serenity::{model::voice::VoiceState, prelude::Context};

use crate::{
    application::models::allowed_ids::AllowedIds,
    extensions::{dependency_ext::Dependencies, log_ext::LogExt},
};

pub async fn handler(ctx: Context, old: Option<VoiceState>, new: VoiceState) -> Result<(), Error> {
    Ok(())
}
