use anyhow::Result;
use mongodb::{Collection, Database};

use crate::application::models::entities::jukebox_use::JukeboxUse;

#[derive(Clone)]
pub struct JukeboxRepository {
    jukebox_use: Collection<JukeboxUse>,
}

impl JukeboxRepository {
    pub fn new(database: &Database) -> Self {
        Self {
            jukebox_use: database.collection("JukeboxUse"),
        }
    }

    pub async fn add_jukebox_use(&self, jukebox_use: JukeboxUse) -> Result<()> {
        self.jukebox_use.insert_one(jukebox_use, None).await?;

        Ok(())
    }
}
