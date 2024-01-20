use anyhow::{anyhow, Error};
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, from_document, Document},
    Collection,
};
use poise::async_trait;
use serde::de::DeserializeOwned;

use super::std_ext::collapse_errors;

#[async_trait]
pub trait CollectionExt<T> {
    async fn random_sample(self, size: u32, filter: Document) -> Result<Vec<T>, Error>;
}

#[async_trait]
impl<T: DeserializeOwned + Send + Sync> CollectionExt<T> for Collection<T> {
    async fn random_sample(self, size: u32, filter: Document) -> Result<Vec<T>, Error> {
        let pipeline = [
            doc! { "$match": filter, },
            doc! { "$sample": { "size": size } },
        ];

        let cursor = self.aggregate(pipeline, None).await?;

        let documents: Vec<_> = cursor.try_collect().await?;

        let entities = documents
            .into_iter()
            .map(|d| from_document(d).map_err(|e| anyhow!(e)));

        collapse_errors(entities)
    }
}
