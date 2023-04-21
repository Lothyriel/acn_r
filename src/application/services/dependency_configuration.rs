use mongodb::Client;
use serenity::prelude::TypeMap;
use tokio::sync::RwLockWriteGuard;

use crate::application::{
    infra::mongo_client::MongoClient,
    models::{allowed_ids::AllowedIds, appsettings::AppSettings},
};

pub fn register_dependencies(
    mut data: RwLockWriteGuard<TypeMap>,
    settings: AppSettings,
    mongo_client: Client,
) {
    data.insert::<AllowedIds>(settings.allowed_ids);
    data.insert::<MongoClient>(mongo_client);
}
