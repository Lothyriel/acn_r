use anyhow::Error;
use serenity::{async_trait, framework::StandardFramework, Client};

use crate::application::{services::{
    appsettings::load_appsettings, dependency_configuration::register_dependencies,
}, infra::mongo_client::create_mongo_client};

pub trait FrameworkExtensions {
    fn register_groups(self) -> StandardFramework;
}

#[async_trait]
pub trait DependenciesExtensions {
    async fn register_dependencies(&mut self) -> Result<(), Error>;
}

#[async_trait]
impl DependenciesExtensions for Client {
    async fn register_dependencies(&mut self) -> Result<(), Error> {
        let settings = load_appsettings()?;
        let mongo_client = create_mongo_client(&settings).await?;

        register_dependencies(self.data.write().await, settings, mongo_client);

        Ok(())
    }
}
