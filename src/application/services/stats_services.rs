use std::collections::HashMap;

use anyhow::{anyhow, Error};
use futures::TryStreamExt;
use mongodb::{bson::doc, options::FindOneOptions, Collection, Database};

use crate::application::models::{
    dto::stats::{StatsDto, UserStats},
    entities::{user::Activity, user_activity::UserActivity},
};

pub struct StatsServices {
    user_activity: Collection<UserActivity>,
}

impl StatsServices {
    pub fn new(database: &Database) -> Self {
        Self {
            user_activity: database.collection("UserActivity"),
        }
    }

    pub async fn get_stats_of_guild(
        &self,
        guild_id: u64,
        target: Option<u64>,
    ) -> Result<StatsDto, Error> {
        let mut filters = vec![
            doc! {"guild_id": guild_id as i64},
            doc! {"activity_type": {"$in": [Activity::Connected.to_string(), Activity::Disconnected.to_string()]}},
        ];

        if let Some(user_id) = target {
            filters.push(doc! {"user_id": user_id as i64});
        }

        let cursor = self
            .user_activity
            .find(doc! {"$and": &filters}, None)
            .await?;

        let guild_activity: Vec<_> = cursor.try_collect().await?;

        let stats_by_user = guild_activity.iter().fold(HashMap::new(), |mut map, e| {
            map.entry(e.user_id).or_insert(Vec::new()).push(e);
            map
        });

        let sort = FindOneOptions::builder().sort(doc! { "date": -1}).build();
        let first_activity = self
            .user_activity
            .find_one(doc! {"$and": &filters}, sort)
            .await?
            .ok_or_else(|| anyhow!("Guild {guild_id} hasn't any data"))?;

        let mut time_by_user: Vec<_> = stats_by_user
            .into_iter()
            .map(|e| get_user_stat(e.0, e.1))
            .collect();

        time_by_user.sort_by(|e1, e2| e2.seconds_online.cmp(&e1.seconds_online));

        let stats = StatsDto {
            initial_date: first_activity.date,
            stats: time_by_user,
        };

        Ok(stats)
    }
}

fn get_user_stat(user_id: u64, activities: Vec<&UserActivity>) -> UserStats {
    let connects = activities
        .iter()
        .filter(|e| e.activity_type == Activity::Connected);

    let disconnects = activities
        .iter()
        .filter(|e| e.activity_type == Activity::Disconnected);

    let zip = connects.into_iter().zip(disconnects);

    let connected_seconds = zip.into_iter().map(|e| {
        let connected = e.0.date;
        let disconnected = e.1.date;

        let time = disconnected - connected;

        time.num_seconds()
    });

    let time_online = connected_seconds.sum();

    UserStats {
        user_id,
        seconds_online: time_online,
    }
}
