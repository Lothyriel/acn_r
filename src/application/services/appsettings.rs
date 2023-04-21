use std::fs;

use anyhow::{anyhow, Error};

use crate::application::models::appsettings::AppSettings;

pub const APPSETTINGS_PATH: &str = "./appsettings.json";

pub fn load_appsettings() -> Result<AppSettings, Error> {
    let settings_path = fs::read_to_string(APPSETTINGS_PATH)?;
    serde_json::from_str::<AppSettings>(&settings_path).map_err(|e| anyhow!(e))
}
