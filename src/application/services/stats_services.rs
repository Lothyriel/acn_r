use std::collections::HashMap;

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

    pub async fn get_stats_of_guild(&self, guild_id: u64) -> Result<Vec<(u64, i64)>, Error> {
        let filter = doc! {"$and": [
            {"guild_id": guild_id as i64},
            {"activity_type": {"$in": [Activity::Connected.to_string(), Activity::Disconnected.to_string()]}}
        ]};

        let cursor = self.user_activity.find(filter, None).await?;
        let guild_activity: Vec<_> = cursor.try_collect().await?;

        let stats_by_user = guild_activity
            .into_iter()
            .fold(HashMap::new(), |mut map, e| {
                map.entry(e.user_id).or_insert(Vec::new()).push(e);
                map
            });

        let mut time_by_user: Vec<_> = stats_by_user
            .into_iter()
            .map(|e| get_online_time(e.0, e.1))
            .collect();

        time_by_user.sort_by(|e1, e2| e2.1.cmp(&e1.1));

        Ok(time_by_user)
    }
}

fn get_online_time(user_id: u64, activities: Vec<UserActivity>) -> (u64, i64) {
    let connects: Vec<_> = activities
        .iter()
        .filter(|e| e.activity_type == Activity::Connected)
        .collect();

    let disconnects: Vec<_> = activities
        .iter()
        .filter(|e| e.activity_type == Activity::Disconnected)
        .collect();

    let zip = connects.into_iter().zip(disconnects);

    let connected_seconds: Vec<_> = zip
        .into_iter()
        .map(|e| {
            let connected = e.0.date;
            let disconnected = e.1.date;

            let time = disconnected - connected;

            time.num_seconds()
        })
        .collect();

    let total_seconds_connected = connected_seconds.into_iter().sum();

    (user_id, total_seconds_connected)
}
