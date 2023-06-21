use anyhow::{anyhow, Error};
use futures::StreamExt;
use mongodb::{
    bson::{doc, from_document},
    Collection, Database,
};
use mongodb_gridfs::{options::GridFSBucketOptions, GridFSBucket};

use crate::application::models::{dto::reaction_dto::AddReactionDto, entities::reaction::Reaction};

#[derive(Clone)]
pub struct ReactionRepository {
    reactions: Collection<Reaction>,
    bucket: GridFSBucket,
}

impl ReactionRepository {
    pub fn new(db: &Database) -> Self {
        let bucket_options = GridFSBucketOptions::builder()
            .bucket_name("Reactions".to_owned())
            .build();

        Self {
            reactions: db.collection("Reactions"),
            bucket: GridFSBucket::new(db.to_owned(), Some(bucket_options)),
        }
    }

    pub async fn add_reaction(&mut self, dto: AddReactionDto) -> Result<(), Error> {
        let id = self
            .bucket
            .upload_from_stream(dto.filename.as_str(), dto.bytes, None)
            .await?;

        let reactions = Reaction {
            id,
            date: chrono::Utc::now(),
            emotion: dto.emotion,
            guild_id: dto.guild_id,
            user_id: dto.user_id,
            filename: dto.filename,
        };

        self.reactions.insert_one(&reactions, None).await?;

        Ok(())
    }

    pub async fn reaction(
        &self,
        emotion: String,
        guild: Option<u64>,
    ) -> Result<(Reaction, Vec<u8>), Error> {
        let reaction = self.get_reaction(emotion, guild).await?;

        let mut stream = self.bucket.open_download_stream(reaction.id).await?;

        let bytes = stream
            .next()
            .await
            .ok_or_else(|| anyhow!("Error downloading file"))?;

        Ok((reaction, bytes))
    }

    async fn get_reaction(&self, emotion: String, guild: Option<u64>) -> Result<Reaction, Error> {
        let pipeline = [
            doc! { "$match": {"emotion": emotion, "guild_id": guild.map(|x| x as i64)} },
            doc! { "$sample": { "size": 1 } },
        ];

        let mut cursor = self.reactions.aggregate(pipeline, None).await?;

        match cursor.advance().await? {
            true => Ok(from_document(cursor.deserialize_current()?)?),
            false => Err(anyhow!("não tem reaçao com essa emoção man.")),
        }
    }
}
