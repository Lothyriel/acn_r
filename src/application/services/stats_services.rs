use std::{collections::HashMap, ops::Not};

use anyhow::{anyhow, Error};
use futures::TryStreamExt;
use log::{error, warn};
use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection, Database,
};
use serenity::async_trait;

use crate::{
    application::models::{
        dto::stats::{StatsDto, UserStats},
        entities::{user::Activity, user_activity::UserActivity},
    },
    extensions::serenity::{guild_ext, serenity_structs::Context},
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

    pub async fn clean_spoiled_stats(
        &self,
        guild_id: u64,
        status_provider: impl OnlineStatusProvider,
    ) -> Result<Vec<ObjectId>, Error> {
        let activities = self.get_activities(guild_id, None).await?;

        let users_online = status_provider.get_status().await?;

        let users_with_discrepancies: Vec<_> = activities
            .into_iter()
            .filter(|(id, act)| {
                let (connects, disconnects): (Vec<_>, Vec<_>) = act
                    .iter()
                    .partition(|a| a.activity_type == Activity::Connected);

                match (connects.len() as i32) - (disconnects.len() as i32) {
                    0 => {
                        let maybe_last = act.last().ok_or_else(|| {
                            anyhow!("IMPOSSIBLE: User {id} got here without any activity")
                        });

                        match maybe_last {
                            Ok(last) => last.activity_type == Activity::Connected,
                            Err(e) => {
                                error!("{e}");
                                false
                            }
                        }
                    }
                    1 => users_online.contains(&id).not(),
                    _ => true,
                }
            })
            .collect();

        let ids = get_spoiled_ids(users_with_discrepancies);

        self.user_activity
            .delete_many(doc! { "_id": { "$in": &ids } }, None)
            .await?;

        Ok(ids)
    }

    pub async fn get_guild_stats(
        &self,
        guild_id: u64,
        target: Option<u64>,
        status_provider: impl OnlineStatusProvider,
    ) -> Result<StatsDto, Error> {
        let cleaned_ids = self.clean_spoiled_stats(guild_id, status_provider).await?;

        warn!("Spoiled activities cleaned: {}", cleaned_ids.len());

        let activities_by_user = self.get_activities(guild_id, target).await?;

        let first_activity_date = activities_by_user
            .iter()
            .flat_map(|a| a.1)
            .min_by(|a, b| a.date.cmp(&b.date))
            .map(|a| a.date)
            .ok_or_else(|| anyhow!("Guild {guild_id} hasn't any data"))?;

        let mut time_by_user: Vec<_> = activities_by_user
            .into_iter()
            .map(|e| get_user_stat(e.0, e.1))
            .collect();

        time_by_user.sort_by(|e1, e2| e2.seconds_online.cmp(&e1.seconds_online));

        let stats = StatsDto {
            initial_date: first_activity_date,
            stats: time_by_user,
        };

        Ok(stats)
    }

    pub async fn get_activities(
        &self,
        guild_id: u64,
        target: Option<u64>,
    ) -> Result<HashMap<u64, Vec<UserActivity>>, Error> {
        let mut filters = vec![
            doc! {"guild_id": guild_id as i64},
            doc! {"activity_type": {"$in": [Activity::Connected.to_string(), Activity::Disconnected.to_string()]}},
        ];

        if let Some(user_id) = target {
            filters.push(doc! {"user_id": user_id as i64});
        }

        let cursor = self
            .user_activity
            .find(doc! {"$and": filters}, None)
            .await?;

        let guild_activity: Vec<_> = cursor.try_collect().await?;

        let stats_by_user = guild_activity
            .into_iter()
            .fold(HashMap::new(), |mut map, e| {
                map.entry(e.user_id).or_insert(Vec::new()).push(e);
                map
            });

        Ok(stats_by_user)
    }
}

fn get_user_stat(user_id: u64, activities: Vec<UserActivity>) -> UserStats {
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

fn get_spoiled_ids(discrepancy: Vec<(u64, Vec<UserActivity>)>) -> Vec<ObjectId> {
    let mut spoiled = vec![];

    for (_, activities) in discrepancy.into_iter() {
        let ids = activities
            .windows(2)
            .filter(|w| w[0].activity_type == w[1].activity_type)
            .map(|w| w[1].id);

        for id in ids {
            spoiled.push(id)
        }
    }

    spoiled
}

#[async_trait]
pub trait OnlineStatusProvider {
    async fn get_status(&self) -> Result<Vec<u64>, Error>;
}

pub struct DiscordOnlineStatus<'a>(pub Context<'a>);

#[async_trait]
impl OnlineStatusProvider for DiscordOnlineStatus<'_> {
    async fn get_status(&self) -> Result<Vec<u64>, Error> {
        let ctx = self.0.serenity_context();

        guild_ext::get_all_online_users(ctx.http.to_owned(), ctx.cache.to_owned()).await
    }
}
