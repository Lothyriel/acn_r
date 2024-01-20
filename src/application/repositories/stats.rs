use anyhow::anyhow;
use futures::TryStreamExt;
use std::collections::HashMap;

use anyhow::Error;
use mongodb::{bson::doc, Collection, Database};

use crate::application::models::{
    dto::{
        stats::{StatsDto, UserStats},
        user::UserActivityDto,
    },
    entities::{russian_roulette::RussianRoulette, user::Activity, user_activity::UserActivity},
};

#[derive(Clone)]
pub struct StatsRepository {
    russian_roulette: Collection<RussianRoulette>,
    user_activity: Collection<UserActivity>,
}

impl StatsRepository {
    pub fn new(database: &Database) -> Self {
        Self {
            user_activity: database.collection("UserActivity"),
            russian_roulette: database.collection("RussianRoulette"),
        }
    }

    pub async fn add_activity(&self, update_dto: &UserActivityDto) -> Result<(), Error> {
        let info = update_dto
            .guild_info
            .as_ref()
            .ok_or_else(|| anyhow!("Expected Guild info when adding activity"))?;

        let activity = update_dto
            .activity
            .ok_or_else(|| anyhow!("Expected Activity when adding activity"))?;

        let activity = UserActivity {
            guild_id: info.guild_id,
            user_id: update_dto.user_id,
            date: update_dto.date,
            activity_type: activity,
        };

        self.user_activity.insert_one(activity, None).await?;
        Ok(())
    }

    pub async fn add_russian_roulette(&self, attempt: RussianRoulette) -> Result<(), Error> {
        self.russian_roulette.insert_one(attempt, None).await?;

        Ok(())
    }

    pub async fn get_guild_stats_2(
        &self,
        _guild_id: u64,
        _target: Option<u64>,
    ) -> Result<StatsDto, Error> {
        todo!()
    }

    pub async fn get_guild_stats(
        &self,
        guild_id: u64,
        target: Option<u64>,
    ) -> Result<StatsDto, Error> {
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
