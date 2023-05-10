use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

use anyhow::{anyhow, Error};
use log::warn;
use mongodb::Database;
use poise::serenity_prelude::Guild;
use serenity::{client::Cache, http::Http};
use tokio::{sync::Semaphore, time::sleep};

use crate::application::infra::appsettings::AppConfigurations;

const SECONDS_IN_5_MINUTES: u64 = 5 * 60;

#[derive(Clone)]
pub struct GithubServices {
    deploy_semaphor: Arc<Semaphore>,
    configurations: Arc<RwLock<AppConfigurations>>,
}

impl GithubServices {
    pub fn new(_database: &Database, configurations: Arc<RwLock<AppConfigurations>>) -> Self {
        Self {
            configurations,
            deploy_semaphor: Arc::new(Semaphore::new(1)),
        }
    }

    pub async fn try_deploy(&self, http: Arc<Http>, cache: Arc<Cache>) -> Result<(), Error> {
        let permit_result = self.deploy_semaphor.acquire().await;

        match permit_result {
            Ok(_) => self.poll_deploy(http, cache).await,
            Err(_) => Ok(()),
        }
    }

    async fn poll_deploy(&self, http: Arc<Http>, cache: Arc<Cache>) -> Result<(), Error> {
        let someone_online = self
            .is_someone_online(http.to_owned(), cache.to_owned())
            .await?;

        match someone_online {
            true => Ok(()),
            false => {
                sleep(Duration::from_secs(SECONDS_IN_5_MINUTES)).await;
                match self.is_someone_online(http, cache).await? {
                    true => Ok(()),
                    false => {
                        self.start_deploy().await?;
                        Ok(())
                    }
                }
            }
        }
    }

    async fn is_someone_online(&self, http: Arc<Http>, cache: Arc<Cache>) -> Result<bool, Error> {
        let guilds = get_guilds(&http, cache).await?;

        let voice_states: Vec<_> = guilds
            .into_iter()
            .flat_map(|g| g.voice_states.into_values())
            .collect();

        let online_count = voice_states
            .iter()
            .filter(|p| p.channel_id.is_some())
            .count();

        warn!("Users online: {}", online_count);

        Ok(online_count > usize::MIN)
    }

    async fn start_deploy(&self) -> Result<(), Error> {
        warn!("Calling Github API and triggering action deploy");
        warn!("(TODO!)");
        Ok(())
    }
}

async fn get_guilds(http: &Arc<Http>, cache: Arc<Cache>) -> Result<Vec<Guild>, Error> {
    let guilds_info = http.get_guilds(None, None).await?;

    let get_guild_results = guilds_info.into_iter().map(|g| {
        g.id.to_guild_cached(&cache)
            .ok_or_else(|| anyhow!("Couldn't get Guild {} from cache", g.id))
    });

    get_guild_results.collect()
}
