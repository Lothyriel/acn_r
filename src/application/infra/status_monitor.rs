use std::{collections::HashSet, sync::Arc, time::Duration};

use anyhow::Error;
use poise::serenity_prelude::{Cache, Http};
use tokio::time;

use crate::{
    application::services::user_services::UserRepository,
    extensions::serenity::guild_ext::{self, UserStatusInfo},
};

pub struct StatusMonitor {
    user_services: UserRepository,
    http: Arc<Http>,
    cache: Arc<Cache>,
}

impl StatusMonitor {
    pub fn new(user_services: UserRepository, http: Arc<Http>, cache: Arc<Cache>) -> Self {
        Self {
            user_services,
            http,
            cache,
        }
    }

    pub async fn monitor_status_loop(&self) -> Result<(), Error> {
        let mut interval = time::interval(Duration::from_secs(60));

        let manager = StatusManager::new(self.get_online_users().await?);

        loop {
            interval.tick().await;

            let new_status = self.get_online_users().await?;

            let update = manager.update_status(new_status);
        }
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

    fn disconnect_user(mut self, user_info: UserStatusInfo) {
        self.current_status.remove(&user_info);
    }

    fn connect_user(mut self, user_info: UserStatusInfo) {
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
