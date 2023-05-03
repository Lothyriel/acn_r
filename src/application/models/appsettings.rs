use serde::{Deserialize, Serialize};
use serenity::prelude::TypeMapKey;

#[derive(Deserialize, Serialize, Debug)]
pub struct AppSettings {
    pub allowed_ids: Vec<u64>,
    pub mongo_user: String,
    pub mongo_cluster_url: String,
}

#[derive(Clone, Copy)]
pub struct AppConfigurations {
    pub debug: bool,
}

impl AppConfigurations {
    pub fn new() -> Self {
        Self { debug: false }
    }
}

impl TypeMapKey for AppConfigurations {
    type Value = AppConfigurations;
}
