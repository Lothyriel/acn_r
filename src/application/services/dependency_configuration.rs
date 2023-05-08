use std::sync::Mutex;

use anyhow::Error;
use mongodb::Database;

use crate::application::{
    models::{
        appsettings::{AppConfigurations, AppSettings},
    },
    services::{
        command_services::CommandServices, guild_services::GuildServices,
        stats_services::StatsServices, user_services::UserServices,
    }, infra::mongo_client::create_mongo_client,
};

pub struct DependencyContainer {
    pub allowed_ids: Vec<u64>,
    pub user_services: UserServices,
    pub guild_services: GuildServices,
    pub command_services: CommandServices,
    pub stats_services: StatsServices,
    pub app_configurations: Mutex<AppConfigurations>,
}

impl DependencyContainer {
    fn new(db: Database, settings: AppSettings) -> Self {
        let guild_services = GuildServices::new(&db);
        let user_services = UserServices::new(&db, guild_services.to_owned());
        let command_services = CommandServices::new(&db, user_services.to_owned());
        let stats_services = StatsServices::new(&db);
        let app_configurations = AppConfigurations::new();

        Self {
            allowed_ids: settings.allowed_ids,
            user_services,
            guild_services,
            command_services,
            stats_services,
            app_configurations: Mutex::new(app_configurations),
        }
    }

    pub async fn build(settings: AppSettings) -> Result<Self, Error> {
        let db = create_mongo_client(&settings).await?.database("acn_r");
        Ok(Self::new(db, settings))
    }
}
