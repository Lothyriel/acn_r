use anyhow::{anyhow, Error};
use mongodb::{
    bson::{doc, from_document},
    Collection, Database,
};

use crate::application::models::entities::reaction::Reaction;

#[derive(Clone)]
pub struct ReactionRepository {
    reactions: Collection<Reaction>,
}

impl ReactionRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            reactions: db.collection("Reactions"),
        }
    }

    pub async fn add_reaction(&self, reaction: Reaction) -> Result<(), Error> {
        self.reactions.insert_one(reaction, None).await?;

        Ok(())
    }

    pub async fn get_reaction(&self, emotion: String, guild_id: Option<u64>) -> Result<Reaction, Error> {
        let pipeline = [
            doc! { "$match": {"emotion": emotion, "guild_id": guild_id.map(|x| x as i64)} },
            doc! { "$sample": { "size": 1 } },
        ];

        let mut cursor = self.reactions.aggregate(pipeline, None).await?;

        match cursor.advance().await? {
            true => Ok(from_document(cursor.deserialize_current()?)?),
            false => Err(anyhow!("não tem reaçao com essa emoção man.")),
        }
    }
}
