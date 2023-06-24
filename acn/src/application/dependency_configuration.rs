use std::sync::Arc;

use anyhow::Error;
use lavalink_rs::LavalinkClient;
use lib::{
    application::{
        dependency_configuration::RepositoriesContainer,
        infra::{
            appsettings::AppConfigurations, deploy_service::DeployServices,
            http_clients::github_client::GithubClient, status_monitor::StatusMonitor,
        },
    },
    extensions::log_ext::LogExt,
};
use poise::serenity_prelude::{Cache, Http, UserId};
use reqwest::Client;
use tokio::sync::RwLock;

use crate::AppSettings;

use super::lavalink_ctx;

pub struct DependencyContainer {
    pub services: ServicesContainer,
    pub repositories: RepositoriesContainer,
}

impl DependencyContainer {
    pub async fn build(
        settings: AppSettings,
        id: UserId,
        http: Arc<Http>,
        cache: Arc<Cache>,
    ) -> Result<Self, Error> {
        let repositories = RepositoriesContainer::build(&settings.mongo_settings).await?;

        let services = ServicesContainer::build(&repositories, settings, id, http, cache).await?;

        Ok(Self {
            services,
            repositories,
        })
    }
}

pub struct ServicesContainer {
    pub bot_id: UserId,
    pub allowed_ids: Vec<u64>,
    pub app_configurations: Arc<RwLock<AppConfigurations>>,
    pub lava_client: LavalinkClient,
    pub status_monitor: Arc<StatusMonitor>,
    pub deploy_services: DeployServices,
}

impl ServicesContainer {
    async fn build(
        repositories: &RepositoriesContainer,
        settings: AppSettings,
        bot_id: UserId,
        http: Arc<Http>,
        cache: Arc<Cache>,
    ) -> Result<Self, Error> {
        let http_client = Client::new();
        let lava_client = lavalink_ctx::get_lavalink_client(&settings.lavalink_settings).await?;

        let github_client = Arc::new(GithubClient::new(http_client, settings.github_settings));

        let app_configurations = Arc::new(RwLock::new(AppConfigurations::new()));

        let deploy_services = DeployServices::new(github_client, app_configurations.to_owned());

        let status_monitor = Arc::new(
            StatusMonitor::new(
                repositories.user.to_owned(),
                http.to_owned(),
                cache.to_owned(),
            )
            .await?,
        );

        let create_loop_task =
            |a: Arc<StatusMonitor>| async move { a.monitor_status_loop().await.log() };

        tokio::spawn(create_loop_task(status_monitor.to_owned()));

        Ok(Self {
            deploy_services,
            lava_client,
            bot_id,
            allowed_ids: settings.allowed_ids,
            app_configurations,
            status_monitor,
        })
    }
}
