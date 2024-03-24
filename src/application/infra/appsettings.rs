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
    pub mongo_settings: MongoSettings,
}

impl AppSettings {
    pub fn load() -> Result<Self> {
        env::init()?;
        let env = env::get("ENV")?;

        let filename = APPSETTINGS_PATH.replace("{ENV}", env.as_str());

        let path = try_get_file(5, filename)?;

        let settings_json = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&settings_json)?)
    }
}

#[derive(Deserialize)]
pub struct MongoSettings {
    pub connection_string: String,
}

impl MongoSettings {
    pub async fn create_mongo_client(settings: &MongoSettings) -> Result<Client> {
        let password = env::get("MONGO_PASSWORD")?;
        let connection_string = settings.connection_string.replace("{PASSWORD}", &password);

        let options = ClientOptions::parse(connection_string).await?;
        Ok(Client::with_options(options)?)
    }
}
