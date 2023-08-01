use anyhow::Error;
use mongodb::{Collection, Database};

use crate::application::models::entities::voice::VoiceSnippet;

#[derive(Clone)]
pub struct VoiceRepository {
    voice_snippets: Collection<VoiceSnippet>,
}

impl VoiceRepository {
    pub fn new(database: &Database) -> Self {
        Self {
            voice_snippets: database.collection("VoiceSnippets"),
        }
    }

    pub async fn add_voice_snippet(&self, snippet: VoiceSnippet) -> Result<(), Error> {
        self.voice_snippets.insert_one(snippet, None).await?;

        Ok(())
    }

    pub async fn get_voice_snippet(&self) -> Result<Option<VoiceSnippet>, Error> {
        Ok(self.voice_snippets.find_one(None, None).await?)
    }
}
