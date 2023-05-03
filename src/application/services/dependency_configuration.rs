use mongodb::Database;
use serenity::prelude::TypeMap;
use tokio::sync::RwLockWriteGuard;

use crate::application::{
    models::{
        allowed_ids::AllowedIds,
        appsettings::{AppConfigurations, AppSettings},
    },
    services::{
        command_services::CommandServices, guild_services::GuildServices,
        stats_services::StatsServices, user_services::UserServices,
    },
};

pub fn register_dependencies(
    mut data: RwLockWriteGuard<TypeMap>,
    settings: AppSettings,
    mongo_database: Database,
) {
    let guild_services = GuildServices::new(&mongo_database);
    let user_services = UserServices::new(&mongo_database, guild_services.to_owned());
    let command_services = CommandServices::new(&mongo_database, user_services.to_owned());
    let stats_services = StatsServices::new(&mongo_database);
    let app_configurations = AppConfigurations::new();

    data.insert::<AllowedIds>(settings.allowed_ids);
    data.insert::<UserServices>(user_services);
    data.insert::<GuildServices>(guild_services);
    data.insert::<CommandServices>(command_services);
    data.insert::<StatsServices>(stats_services);
    data.insert::<AppConfigurations>(app_configurations);
}
