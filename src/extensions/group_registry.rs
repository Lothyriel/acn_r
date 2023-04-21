use anyhow::Error;
use serenity::{async_trait, framework::StandardFramework, Client};

use crate::application::{
    infra::mongo_client::create_mongo_client, models::appsettings::AppSettings,
    services::dependency_configuration::register_dependencies,
};

pub trait FrameworkExtensions {
    fn register_groups(self) -> StandardFramework;
}

#[async_trait]
pub trait DependenciesExtensions {
    async fn register_dependencies(&mut self, settings: AppSettings) -> Result<(), Error>;
}

#[async_trait]
impl DependenciesExtensions for Client {
    async fn register_dependencies(&mut self, settings: AppSettings) -> Result<(), Error> {
        let mongo_client = create_mongo_client(&settings).await?;

        register_dependencies(self.data.write().await, settings, mongo_client);

        Ok(())
    }
}
