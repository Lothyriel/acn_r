use serde::{Deserialize, Serialize};

use crate::application::models::db_file::DbFile;

#[derive(Serialize, Deserialize)]
pub struct Reaction {
    pub emotion: String,
    pub guild_id: Option<u64>,
    pub user_id: u64,
    pub file: DbFile,
}
