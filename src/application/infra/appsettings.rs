use anyhow::{bail, Result};
use mongodb::{options::ClientOptions, Client};
use poise::serenity_prelude::UserId;
use serde::Deserialize;
use std::path::PathBuf;

use crate::application::infra::env;

const APPSETTINGS_PATH: &str = "appsettings_{ENV}.json";

fn try_get_file(max_depth: usize, filename: String) -> Result<PathBuf> {
    for i in 0..max_depth {
        let try_path = format!("{}{}", "../".repeat(i), &filename);
        let possible_path = std::path::Path::new(&try_path);

        match possible_path.exists() {
            true => return Ok(possible_path.to_path_buf()),
            false => continue,
        }
    }

    bail!("The file {} was not found in depth {}", filename, max_depth)
}

#[derive(Deserialize)]
pub struct AppSettings {
    pub allowed_ids: Vec<UserId>,
    pub prefix: String,
}

impl AppSettings {
    pub fn load() -> Result<Self> {
        env::init()?;
        let env = env::get("ENV").unwrap_or_else(|_| "dev".to_string());

        let filename = APPSETTINGS_PATH.replace("{ENV}", env.as_str());

        let path = try_get_file(5, filename)?;

        let settings_json = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&settings_json)?)
    }
}

pub async fn create_mongo_client() -> Result<Client> {
    let connection_string = env::get("MONGO_CONNECTION_STRING")
        .unwrap_or_else(|_| "mongodb://localhost/?retryWrites=true".to_owned());

    let options = ClientOptions::parse(connection_string).await?;

    Ok(Client::with_options(options)?)
}
