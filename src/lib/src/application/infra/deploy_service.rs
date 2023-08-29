use anyhow::Error;
use log::warn;
use poise::serenity_prelude::{Cache, Http};
use std::{sync::Arc, time::Duration};
use tokio::{
    sync::{RwLock, Semaphore},
    time,
};

use crate::{
    application::infra::{
        appsettings::AppConfigurations, http_clients::github_client::GithubClient,
    },
    extensions::serenity::guild_ext,
};

const SECONDS_IN_5_MINUTES: u64 = 5 * 60;

#[derive(Clone)]
pub struct DeployServices {
    deploy_semaphor: Arc<Semaphore>,
    configurations: Arc<RwLock<AppConfigurations>>,
    client: Arc<GithubClient>,
    deploy_file: String,
}

impl DeployServices {
    pub fn new(
        client: Arc<GithubClient>,
        configurations: Arc<RwLock<AppConfigurations>>,
        deploy_file: String,
    ) -> Self {
        Self {
            deploy_file,
            client,
            configurations,
            deploy_semaphor: Arc::new(Semaphore::new(1)),
        }
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
        let someone_online = is_someone_online(http.to_owned(), cache.to_owned()).await?;

        match someone_online {
            true => Ok(()),
            false => {
                warn!("Deploying in {} seconds", SECONDS_IN_5_MINUTES);
                time::sleep(Duration::from_secs(SECONDS_IN_5_MINUTES)).await;
                match is_someone_online(http, cache).await? {
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

    async fn start_deploy(&self) -> Result<(), Error> {
        warn!("Calling Github API and triggering action deploy");
        self.client.deploy(&self.deploy_file).await
    }
}

async fn is_someone_online(http: Arc<Http>, cache: Arc<Cache>) -> Result<bool, Error> {
    let online_users = guild_ext::get_all_online_users(http, cache).await?;

    let count = online_users.count();

    warn!("Users online: {}", count);

    Ok(count > usize::MIN)
}
