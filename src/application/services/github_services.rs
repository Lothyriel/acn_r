use std::{sync::Arc, time::Duration};

use anyhow::{anyhow, Error};
use futures::future::join_all;
use log::info;
use mongodb::Database;
use serenity::http::Http;
use tokio::{sync::Semaphore, time::sleep};

use crate::extensions::std_ext::VecResultErrorExt;

const SECONDS_IN_5_MINUTES: u64 = 5 * 60;

#[derive(Clone)]
pub struct GithubServices {
    deploy_semaphor: Arc<Semaphore>,
}

impl GithubServices {
    pub fn new(_database: &Database) -> Self {
        Self {
            deploy_semaphor: Arc::new(Semaphore::new(1)),
        }
    }

    pub async fn try_deploy(&self, http: Arc<Http>) -> Result<bool, Error> {
        match self.is_someone_online(http.to_owned()).await? {
            true => Ok(false),
            false => {
                sleep(Duration::from_secs(SECONDS_IN_5_MINUTES)).await;
                match self.is_someone_online(http).await? {
                    true => Ok(false),
                    false => {
                        self.start_deploy().await?;
                        Ok(true)
                    }
                }
            }
        }
    }

    async fn is_someone_online(&self, http: Arc<Http>) -> Result<bool, Error> {
        let guilds_info = http.get_guilds(None, None).await?;
        let tasks_get_guild: Vec<_> = guilds_info
            .into_iter()
            .map(|g| http.get_guild(g.id.0))
            .collect();

        let get_guild_results: Vec<_> = join_all(tasks_get_guild)
            .await
            .into_iter()
            .map(|t| t.map_err(|e| anyhow!(e)))
            .collect();

        let guilds = get_guild_results.all_successes()?;

        let presence_count_results: Vec<_> = guilds
            .into_iter()
            .map(|g| {
                g.approximate_presence_count
                    .ok_or_else(|| anyhow!("Error getting presence count of: {}", g.id))
            })
            .collect();

        let presence_counts = presence_count_results.all_successes()?;
        Ok(presence_counts.iter().any(|p| p > &u64::MIN))
    }

    async fn start_deploy(&self) -> Result<(), Error> {
        info!("(TODO!) Should call Github API and trigger the action deploy");
        Ok(())
    }
}
