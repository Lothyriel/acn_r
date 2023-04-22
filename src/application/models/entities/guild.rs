use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Guild {
    pub id: u64,
}
