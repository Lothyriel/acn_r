use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Reaction {
    pub emotion: String,
    pub guild_id: Option<u64>,
    pub user_id: u64,
    pub file_name: String,
    pub bytes: Vec<u8>
}