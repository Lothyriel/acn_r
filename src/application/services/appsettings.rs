use std::fs;

use crate::application::models::appsettings::AppSettings;

pub const APPSETTINGS_PATH: &str = "./appsettings.json";

pub fn load_appsettings() -> AppSettings {
    let settings_path = fs::read_to_string(APPSETTINGS_PATH).expect("Não achei a appsettings");
    serde_json::from_str::<AppSettings>(&settings_path).expect("Não é um json vei")
}
