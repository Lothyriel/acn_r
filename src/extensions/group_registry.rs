use serenity::{async_trait, framework::StandardFramework, Client};

use crate::application::services::{
    appsettings::load_appsettings, dependency_configuration::register_dependencies,
};

pub trait FrameworkExtensions {
    fn register_groups(self) -> StandardFramework;
}

#[async_trait]
pub trait DependenciesExtensions {
    async fn register_dependencies(&mut self);
}

#[async_trait]
impl DependenciesExtensions for Client {
    async fn register_dependencies(&mut self) {
        register_dependencies(self.data.write().await, load_appsettings())
    }
}
