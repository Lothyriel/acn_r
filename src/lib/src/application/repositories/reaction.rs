use anyhow::{anyhow, Error};
use futures::StreamExt;
use mongodb::{
    bson::{doc, from_document},
    Collection, Database,
};
use mongodb_gridfs::{options::GridFSBucketOptions, GridFSBucket};

use crate::application::models::{
    dto::reaction_dto::AddReactionDto,
    entities::reaction::{Reaction, ReactionUse},
};

#[derive(Clone)]
pub struct ReactionRepository {
    reactions: Collection<Reaction>,
    reactions_use: Collection<ReactionUse>,
    bucket: GridFSBucket,
}

impl ReactionRepository {
    pub fn new(db: &Database) -> Self {
        let bucket_options = GridFSBucketOptions::builder()
            .bucket_name("Reactions".to_owned())
            .build();

        Self {
            reactions_use: db.collection("ReactionsUse"),
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
            date_added: chrono::Utc::now(),
            emotion: dto.emotion,
            guild_id: dto.guild_id,
            creator_id: dto.user_id,
            filename: dto.filename,
        };

        self.reactions.insert_one(&reactions, None).await?;

        Ok(())
    }

    pub async fn reaction(
        &self,
        emotion: Option<String>,
        guild_id: u64,
        user_id: u64,
    ) -> Result<(Reaction, Vec<u8>), Error> {
        let reaction = self.get_reaction(emotion, guild_id).await?;

        let reaction_use = ReactionUse {
            reaction_id: reaction.id,
            date: chrono::Utc::now(),
            user_id,
        };

        self.reactions_use.insert_one(reaction_use, None).await?;

        let mut stream = self.bucket.open_download_stream(reaction.id).await?;

        let mut bytes = vec![];

        while let Some(chunk) = stream.next().await {
            bytes.extend_from_slice(&chunk);
        }

        Ok((reaction, bytes))
    }

    pub async fn list(&self, guild_id: u64) -> Result<impl Iterator<Item = String>, Error> {
        let emotions = self
            .reactions
            .distinct("emotion", doc! {"guild_id": guild_id as i64}, None)
            .await?;

        Ok(emotions.into_iter().map(|emotion| emotion.to_string()))
    }

    async fn get_reaction(
        &self,
        emotion: Option<String>,
        guild_id: u64,
    ) -> Result<Reaction, Error> {
        let mut filter = doc! { "guild_id": guild_id as i64};

        if let Some(emotion) = emotion {
            filter.insert("emotion", emotion);
        }

        let pipeline = [
            doc! { "$match": filter, },
            doc! { "$sample": { "size": 1 } },
        ];

        let mut cursor = self.reactions.aggregate(pipeline, None).await?;

        match cursor.advance().await? {
            true => Ok(from_document(cursor.deserialize_current()?)?),
            false => Err(anyhow!("Sem emoções correspondentes registradas")),
        }
    }
}
