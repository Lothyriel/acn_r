use std::{collections::HashSet, sync::Arc, time::Duration};

use anyhow::Error;
use poise::serenity_prelude::{Cache, Http};
use tokio::{sync::Mutex, time};

use crate::{
    application::{models::dto::user::UpdateActivityDto, repositories::user::UserRepository},
    extensions::serenity::guild_ext::{self, UserStatusInfo},
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

            self.update_status().await?;
        }
    }

    pub async fn update_user_activity(&self, dto: UpdateActivityDto) -> Result<(), Error> {
        self.user_repository.update_user_activity(dto).await?;

        Ok(())
    }

    async fn update_status(&self) -> Result<(), Error> {
        let new_status = self.get_online_users().await?;

        let mut manager = self.manager.lock().await;

        let update = manager.update_status(new_status);

        for c in update.connected {
            manager.connect_user(c);
        }

        for d in update.disconnected {
            manager.disconnect_user(d);
        }

        Ok(())
    }

    async fn get_online_users(&self) -> Result<HashSet<UserStatusInfo>, Error> {
        guild_ext::get_all_online_users(self.http.to_owned(), self.cache.to_owned()).await
    }
}

pub struct StatusManager {
    current_status: HashSet<UserStatusInfo>,
}

impl StatusManager {
    pub fn new(current_status: HashSet<UserStatusInfo>) -> Self {
        Self { current_status }
    }

    pub fn update_status(&self, new_status: HashSet<UserStatusInfo>) -> StatusUpdate {
        let connected = new_status.difference(&self.current_status).cloned();

        let disconnected = self.current_status.difference(&new_status).cloned();

        StatusUpdate::new(connected.collect(), disconnected.collect())
    }

    fn disconnect_user(&mut self, user_info: UserStatusInfo) {
        self.current_status.remove(&user_info);
    }

    fn connect_user(&mut self, user_info: UserStatusInfo) {
        self.current_status.insert(user_info);
    }
}

#[derive(Debug)]
pub struct StatusUpdate {
    pub connected: HashSet<UserStatusInfo>,
    pub disconnected: HashSet<UserStatusInfo>,
}

impl StatusUpdate {
    fn new(connected: HashSet<UserStatusInfo>, disconnected: HashSet<UserStatusInfo>) -> Self {
        Self {
            connected,
            disconnected,
        }
    }
}
