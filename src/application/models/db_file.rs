use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DbFile {
    pub filename: String,
    pub id: ObjectId,
}

