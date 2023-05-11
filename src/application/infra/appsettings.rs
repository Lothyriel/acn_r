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
    pub prefix: String,
    pub mongo_settings: MongoSettings,
    pub github_settings: GithubSettings,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MongoSettings {
    pub user: String,
    pub url: String,
    pub connection_string: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GithubSettings {
    pub username: String,
    pub repository: String,
    pub workflow_file: String,
    pub branch_name: String,
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
