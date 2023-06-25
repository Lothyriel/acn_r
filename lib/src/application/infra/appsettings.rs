use anyhow::Error;
use serde::{de::DeserializeOwned, Deserialize};
use std::fs;

use crate::application::infra::env;

const APPSETTINGS_PATH: &str = "./appsettings_{ENV}.json";

pub fn load<T: DeserializeOwned>() -> Result<T, Error> {
    env::init()?;
    let env = env::get("ENV")?;
    let settings_path = fs::read_to_string(APPSETTINGS_PATH.replace("{ENV}", env.as_str()))?;
    Ok(serde_json::from_str(&settings_path)?)
}

#[derive(Deserialize)]
pub struct AcnSettings {
    pub allowed_ids: Vec<u64>,
    pub prefix: String,
    pub lavalink_settings: LavalinkSettings,
    pub mongo_settings: MongoSettings,
    pub github_settings: GithubSettings,
}

#[derive(Deserialize)]
pub struct TestSettings {
    pub mongo_settings: MongoSettings,
    pub github_settings: GithubSettings,
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
    pub workflow_file: String,
    pub branch_name: String,
}

#[derive(Deserialize)]
pub struct LavalinkSettings {
    pub url: String,
    pub port: u16,
}

pub struct AppConfigurations {
    pub deploy_ready: bool,
}

impl AppConfigurations {
    pub fn new() -> Self {
        Self {
            deploy_ready: false,
        }
    }
}
