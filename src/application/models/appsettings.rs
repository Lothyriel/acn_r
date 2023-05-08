use serde::{Deserialize, Serialize};

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
