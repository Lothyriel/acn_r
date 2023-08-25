use anyhow::{anyhow, Error};
use serde::Deserialize;
use std::path::PathBuf;

use crate::application::infra::env;

const APPSETTINGS_PATH: &str = "appsettings_{ENV}.json";

fn try_get_file(max_depth: usize, filename: String) -> Result<PathBuf, Error> {
    for i in 0..max_depth {
        let try_path = format!("{}{}", "../".repeat(i), &filename);
        let possible_path = std::path::Path::new(&try_path);

        match possible_path.exists() {
            true => return Ok(possible_path.to_path_buf()),
            false => continue,
        }
    }

    let error = anyhow!("The file {} was not found in depth {}", filename, max_depth);

    Err(error)
}

#[derive(Deserialize)]
pub struct AppSettings {
    pub allowed_ids: Vec<u64>,
    pub prefix: String,
    pub lavalink_settings: LavalinkSettings,
    pub mongo_settings: MongoSettings,
    pub github_settings: GithubSettings,
}

impl AppSettings {
    pub fn load() -> Result<Self, Error> {
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
    pub user: String,
    pub url: String,
    pub connection_string: String,
}

#[derive(Deserialize)]
pub struct GithubSettings {
    pub username: String,
    pub repository: String,
    pub branch_name: String,
}

#[derive(Deserialize)]
pub struct LavalinkSettings {
    pub url: String,
    pub port: u16,
}

#[derive(Default)]
pub struct AppConfigurations {
    pub deploy_ready: bool,
}
