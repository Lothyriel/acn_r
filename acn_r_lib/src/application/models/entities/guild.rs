use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Guild {
    pub id: u64,
}
