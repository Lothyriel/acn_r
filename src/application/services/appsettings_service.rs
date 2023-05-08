use std::fs;
use anyhow::Error;

use crate::application::{models::appsettings::AppSettings, infra::env_var};

pub const APPSETTINGS_PATH: &str = "./appsettings_{ENV}.json";

pub fn load() -> Result<AppSettings, Error> {
    let env = env_var::get("ENV")?;
    let settings_path = fs::read_to_string(APPSETTINGS_PATH.replace("{ENV}", env.as_str()))?;
    Ok(serde_json::from_str::<AppSettings>(&settings_path)?)
}
