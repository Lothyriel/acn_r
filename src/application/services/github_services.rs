use std::{sync::Arc, time::Duration};

use anyhow::{anyhow, Error};
use log::warn;
use poise::serenity_prelude::Guild;
use serenity::{client::Cache, http::Http};
use tokio::{
    sync::{RwLock, Semaphore},
    time::sleep,
};

use crate::application::infra::{
    appsettings::AppConfigurations, http_clients::github_client::GithubClient,
};

const SECONDS_IN_30_MINUTES: u64 = 30 * 60;

#[derive(Clone)]
pub struct GithubServices {
    deploy_semaphor: Arc<Semaphore>,
    configurations: Arc<RwLock<AppConfigurations>>,
    client: Arc<GithubClient>,
}

impl GithubServices {
    pub fn build(
        client: Arc<GithubClient>,
        configurations: Arc<RwLock<AppConfigurations>>,
    ) -> Result<Self, Error> {
        Ok(Self {
            client,
            configurations,
            deploy_semaphor: Arc::new(Semaphore::new(1)),
        })
    }

    pub async fn try_deploy(&self, http: Arc<Http>, cache: Arc<Cache>) -> Result<(), Error> {
        let deploy_ready = {
            let configurations = self.configurations.read().await;
            configurations.deploy_ready
        };

        if !deploy_ready {
            return Ok(());
        }

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
                warn!("Deploying in {SECONDS_IN_30_MINUTES} seconds");
                sleep(Duration::from_secs(SECONDS_IN_30_MINUTES)).await;
                match self.is_someone_online(http, cache).await? {
                    true => {
                        warn!("Deploy cancelled");
                        Ok(())
                    }
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
        self.client.deploy().await
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
