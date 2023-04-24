use anyhow::Error;
use futures::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};
use serenity::prelude::TypeMapKey;

use crate::application::models::entities::{user::Activity, user_activity::UserActivity};

impl TypeMapKey for StatsServices {
    type Value = StatsServices;
}

#[derive(Clone)]
pub struct StatsServices {
    user_activity: Collection<UserActivity>,
}

impl StatsServices {
    pub fn new(database: &Database) -> Self {
        Self {
            user_activity: database.collection("UserActivity"),
        }
    }

    pub async fn get_stats_by_guild(&self, guild_id: u64) -> Result<Vec<UserActivity>, Error> {
        let filter = doc! {"$and": [
            {"guild_id": guild_id as i64},
            {"activity_type": {"$in": [Activity::Connected.to_string(), Activity::Disconnected.to_string()]}}
        ]};

        let cursor = self.user_activity.find(filter, None).await?;
        Ok(cursor.try_collect().await?)
    }
}
