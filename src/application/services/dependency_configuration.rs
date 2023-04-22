use mongodb::Database;
use serenity::prelude::TypeMap;
use tokio::sync::RwLockWriteGuard;

use crate::application::models::{allowed_ids::AllowedIds, appsettings::AppSettings};

use super::mongo::{guild_services::GuildServices, user_services::UserServices};

pub fn register_dependencies(
    mut data: RwLockWriteGuard<TypeMap>,
    settings: AppSettings,
    mongo_database: Database,
) {
    let guild_services = GuildServices::new(&mongo_database);
    let user_services = UserServices::new(&mongo_database, guild_services.to_owned());

    data.insert::<AllowedIds>(settings.allowed_ids);
    data.insert::<UserServices>(user_services);
    data.insert::<GuildServices>(guild_services);
}
