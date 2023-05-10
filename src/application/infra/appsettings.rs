use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::fs;

use crate::application::infra::env;

pub const APPSETTINGS_PATH: &str = "./appsettings_{ENV}.json";

pub fn load() -> Result<AppSettings, Error> {
    let env = env::get("ENV")?;
    let settings_path = fs::read_to_string(APPSETTINGS_PATH.replace("{ENV}", env.as_str()))?;
    Ok(serde_json::from_str::<AppSettings>(&settings_path)?)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AppSettings {
    pub allowed_ids: Vec<u64>,
    pub mongo_user: String,
    pub mongo_connection_string: String,
    pub mongo_url: String,
    pub prefix: String,
}

pub struct AppConfigurations {
    pub debug: bool,
}

impl AppConfigurations {
    pub fn new() -> Self {
        Self { debug: false }
    }
}
