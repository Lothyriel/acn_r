use serenity::prelude::TypeMap;
use tokio::sync::RwLockWriteGuard;

use crate::application::{
    models::{allowed_ids::AllowedIds, appsettings::AppSettings},
};

pub fn register_dependencies(mut data: RwLockWriteGuard<TypeMap>, settings: AppSettings) {
    data.insert::<AllowedIds>(settings.allowed_ids);
}
