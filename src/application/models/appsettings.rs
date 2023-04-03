use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AppSettings {
    pub allowed_ids: Vec<u64>,
}