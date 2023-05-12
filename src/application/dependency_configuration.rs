use anyhow::Error;
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::application::{
    infra::{
        appsettings::{AppConfigurations, AppSettings},
        mongo_client::create_mongo_client,
    },
    services::{
        command_services::CommandServices, github_services::GithubServices,
        guild_services::GuildServices, stats_services::StatsServices, user_services::UserServices,
    },
};

use super::infra::http_clients::github_client::GithubClient;

pub struct DependencyContainer {
    pub allowed_ids: Vec<u64>,
    pub app_configurations: Arc<RwLock<AppConfigurations>>,
    pub user_services: UserServices,
    pub command_services: CommandServices,
    pub guild_services: GuildServices,
    pub stats_services: StatsServices,
    pub github_services: GithubServices,
}

impl DependencyContainer {
    pub async fn build(settings: AppSettings) -> Result<Self, Error> {
        let db = create_mongo_client(&settings.mongo_settings)
            .await?
            .database("acn_r");

        let client = Client::new();
        let github_client = Arc::new(GithubClient::new(client, settings.github_settings));

        let configurations = Arc::new(RwLock::new(AppConfigurations::new()));
        let guild_services = GuildServices::new(&db);
        let user_services = UserServices::new(&db, guild_services.to_owned());
        let command_services = CommandServices::new(&db, user_services.to_owned());
        let stats_services = StatsServices::new(&db);
        let github_services = GithubServices::build(github_client, configurations.to_owned())?;

        Ok(Self {
            allowed_ids: settings.allowed_ids,
            app_configurations: configurations,
            user_services,
            guild_services,
            command_services,
            stats_services,
            github_services,
        })
    }
}
