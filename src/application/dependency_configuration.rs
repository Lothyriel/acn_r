use anyhow::Error;
use lavalink_rs::LavalinkClient;
use mongodb::Database;
use poise::serenity_prelude::{Cache, Http};
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    application::{
        infra::{
            appsettings::{AppConfigurations, AppSettings},
            http_clients::github_client::GithubClient,
            mongo_client::create_mongo_client,
            status_monitor::StatusMonitor,
        },
        services::{
            command_services::CommandRepository, github_services::DeployServices,
            guild_services::GuildRepository, jukebox_services::JukeboxRepository,
            stats_services::StatsRepository, user_services::UserRepository,
        },
    },
    extensions::log_ext::LogExt,
};

use super::infra::lavalink_ctx;

pub struct DependencyContainer {
    pub services: ServicesContainer,
    pub repositories: RepositoriesContainer,
}

impl DependencyContainer {
    pub async fn build(
        settings: AppSettings,
        id: u64,
        http: Arc<Http>,
        cache: Arc<Cache>,
    ) -> Result<Self, Error> {
        let repositories = RepositoriesContainer::build(&settings).await?;

        let services = ServicesContainer::build(&repositories, settings, id, http, cache).await?;

        Ok(Self {
            services,
            repositories,
        })
    }
}

pub struct ServicesContainer {
    pub id: u64,
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
        id: u64,
        http: Arc<Http>,
        cache: Arc<Cache>,
    ) -> Result<Self, Error> {
        let http_client = Client::new();
        let lava_client = lavalink_ctx::get_lavalink_client(&settings).await?;

        let github_client = Arc::new(GithubClient::new(http_client, settings.github_settings));

        let app_configurations = Arc::new(RwLock::new(AppConfigurations::new()));

        let deploy_services = DeployServices::new(github_client, app_configurations.to_owned());

        let status_monitor = Arc::new(StatusMonitor::new(
            repositories.user.to_owned(),
            http.to_owned(),
            cache.to_owned(),
        ));

        let task = |a: Arc<StatusMonitor>| async move { a.monitor_status_loop().await.log() };

        tokio::spawn(task(status_monitor.to_owned()));

        Ok(Self {
            deploy_services,
            lava_client,
            id,
            allowed_ids: settings.allowed_ids,
            app_configurations,
            status_monitor,
        })
    }
}

pub struct RepositoriesContainer {
    pub user: UserRepository,
    pub command: CommandRepository,
    pub guild: GuildRepository,
    pub stats: StatsRepository,
    pub jukebox: JukeboxRepository,
}

impl RepositoriesContainer {
    pub async fn build(settings: &AppSettings) -> Result<Self, Error> {
        let db = Self::database(settings).await?;
        Ok(Self::build_with_db(db))
    }

    pub fn build_with_db(db: Database) -> Self {
        let guild = GuildRepository::new(&db);

        let user = UserRepository::new(&db, guild.to_owned());

        let command = CommandRepository::new(&db, user.to_owned());

        let stats = StatsRepository::new(&db);

        let jukebox = JukeboxRepository::new(&db);

        Self {
            user,
            guild,
            command,
            stats,
            jukebox,
        }
    }

    pub async fn database(settings: &AppSettings) -> Result<Database, Error> {
        Ok(create_mongo_client(&settings.mongo_settings)
            .await?
            .database("acn_r"))
    }
}
