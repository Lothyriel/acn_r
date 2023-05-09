use anyhow::Error;
use mongodb::Database;
use std::sync::RwLock;

use crate::application::{
    infra::mongo_client::create_mongo_client,
    models::appsettings::{AppConfigurations, AppSettings},
    services::{
        command_services::CommandServices, github_services::GithubServices,
        guild_services::GuildServices, stats_services::StatsServices, user_services::UserServices,
    },
};

pub struct DependencyContainer {
    pub allowed_ids: Vec<u64>,
    pub app_configurations: RwLock<AppConfigurations>,
    pub user_services: UserServices,
    pub command_services: CommandServices,
    pub guild_services: GuildServices,
    pub stats_services: StatsServices,
    pub github_services: GithubServices,
}

impl DependencyContainer {
    fn new(db: Database, settings: AppSettings) -> Self {
        let guild_services = GuildServices::new(&db);
        let user_services = UserServices::new(&db, guild_services.to_owned());
        let command_services = CommandServices::new(&db, user_services.to_owned());
        let stats_services = StatsServices::new(&db);
        let app_configurations = AppConfigurations::new();
        let github_services = GithubServices::new(&db);

        Self {
            app_configurations: RwLock::new(app_configurations),
            allowed_ids: settings.allowed_ids,
            github_services,
        }
    }

    pub async fn build(settings: AppSettings) -> Result<Self, Error> {
        let db = create_mongo_client(&settings).await?.database("acn_r");
        Ok(Self::new(db, settings))
    }
}
