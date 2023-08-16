use anyhow::Error;
use mongodb::{
    bson::{doc, from_document},
    Collection, Database,
};

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

    pub async fn get_voice_snippet(
        &self,
        guild_id: u64,
        user_id: Option<u64>,
    ) -> Result<Option<VoiceSnippet>, Error> {
        let mut filter = doc! { "guild_id": guild_id as i64};

        if let Some(user_id) = user_id {
            filter.insert("user_id", user_id as i64);
        }

        let pipeline = [
            doc! { "$match": filter, },
            doc! { "$sample": { "size": 1 } },
        ];

        let mut cursor = self.voice_snippets.aggregate(pipeline, None).await?;

        let maybe_snippet = match cursor.advance().await? {
            true => Some(from_document(cursor.deserialize_current()?)?),
            false => None,
        };

        Ok(maybe_snippet)
    }
}
