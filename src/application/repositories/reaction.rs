use anyhow::{anyhow, Error};
use futures::{AsyncReadExt, AsyncWriteExt};
use mongodb::{
    bson::{doc, from_document, Bson},
    options::GridFsBucketOptions,
    Collection, Database, GridFsBucket,
};

use crate::application::models::{dto::reaction_dto::ReactionDto, entities::reaction::Reaction};

#[derive(Clone)]
pub struct ReactionRepository {
    reactions: Collection<Reaction>,
    bucket: GridFsBucket,
}

impl ReactionRepository {
    pub fn new(db: &Database) -> Self {
        let bucket_options = GridFsBucketOptions::builder()
            .bucket_name("Reactions".to_owned())
            .build();

        Self {
            reactions: db.collection("Reactions"),
            bucket: db.gridfs_bucket(bucket_options),
        }
    }

    pub async fn add_reaction(&self, dto: ReactionDto) -> Result<(), Error> {
        let id = Bson::ObjectId(dto.reaction.file.id);

        let mut stream =
            self.bucket
                .open_upload_stream_with_id(id, &dto.reaction.file.filename, None);

        stream.write_all(&dto.bytes).await?;

        self.reactions.insert_one(dto.reaction, None).await?;

        Ok(())
    }

    pub async fn reaction(
        &self,
        emotion: String,
        guild: Option<u64>,
    ) -> Result<ReactionDto, Error> {
        let reaction = self.get_reaction(emotion, guild).await?;

        let mut stream = self
            .bucket
            .open_download_stream(Bson::ObjectId(reaction.file.id))
            .await?;

        let mut buffer = vec![];
        stream.read_to_end(&mut buffer).await?;

        Ok(ReactionDto {
            reaction,
            bytes: buffer,
        })
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
