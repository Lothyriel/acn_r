use std::{collections::HashSet, sync::Arc, time::Duration};

use anyhow::Error;
use futures::future::join_all;
use log::warn;
use poise::serenity_prelude::{Cache, Http};
use tokio::{sync::Mutex, time};

use crate::{
    application::{
        models::{
            dto::user::UpdateActivityDto,
            entities::{user::Activity, user_activity::UserActivity},
        },
        repositories::user::UserRepository,
    },
    extensions::{
        log_ext::LogErrorsExt,
        serenity::guild_ext::{self, StatusInfo},
    },
};

pub struct StatusMonitor {
    user_repository: UserRepository,
    http: Arc<Http>,
    cache: Arc<Cache>,
    manager: Arc<Mutex<StatusManager>>,
}

impl StatusMonitor {
    pub async fn new(
        user_repository: UserRepository,
        http: Arc<Http>,
        cache: Arc<Cache>,
    ) -> Result<Self, Error> {
        let manager = Arc::new(Mutex::new(StatusManager::new(
            guild_ext::get_all_online_users(http.to_owned(), cache.to_owned()).await?,
        )));

        Ok(Self {
            user_repository,
            http,
            cache,
            manager,
        })
    }

    pub async fn monitor_status_loop(&self) -> Result<(), Error> {
        let mut interval = time::interval(Duration::from_secs(60));

        loop {
            interval.tick().await;

            self.update_pending_status().await?;
        }
    }

    pub async fn update_user_activity(&self, dto: UpdateActivityDto) -> Result<(), Error> {
        let mut manager = self.manager.lock().await;

        let user_status = StatusInfo::new(dto.user_id, dto.guild_id);

        match dto.activity {
            Activity::Connected => manager.connect_user(&user_status),
            Activity::Disconnected => manager.disconnect_user(&user_status),
            _ => {}
        }

        self.user_repository.update_user_activity(dto).await?;

        Ok(())
    }

    async fn update_pending_status(&self) -> Result<(), Error> {
        let new_status = self.get_online_users().await?;

        let mut manager = self.manager.lock().await;

        let activities = manager.update_status(new_status);

        let add_activity_tasks = activities.into_iter().map(|a| {
            warn!("Added activity mannualy: {:?}", a);
            self.user_repository.add_activity(a)
        });

        join_all(add_activity_tasks).await.log_errors();

        Ok(())
    }

    async fn get_online_users(&self) -> Result<HashSet<StatusInfo>, Error> {
        guild_ext::get_all_online_users(self.http.to_owned(), self.cache.to_owned()).await
    }
}

pub struct StatusManager {
    current_status: HashSet<StatusInfo>,
}

impl StatusManager {
    pub fn new(current_status: HashSet<StatusInfo>) -> Self {
        Self { current_status }
    }

    pub fn update_status(&mut self, new_status: HashSet<StatusInfo>) -> Vec<UserActivity> {
        let current = self.current_status.to_owned();

        let connected = new_status.difference(&current);

        let disconnected = current.difference(&new_status);

        let mut activities = vec![];

        for c in connected {
            self.connect_user(c);
            activities.push(c.to_activity(Activity::Connected));
        }

        for d in disconnected {
            self.disconnect_user(d);
            activities.push(d.to_activity(Activity::Disconnected));
        }

        activities
    }

    fn disconnect_user(&mut self, user_info: &StatusInfo) {
        self.current_status.remove(user_info);
    }

    fn connect_user(&mut self, user_info: &StatusInfo) {
        self.current_status.insert(user_info.to_owned());
    }
}
