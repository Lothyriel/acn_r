use anyhow::Result;
use mongodb::{options::ClientOptions, Client};

use crate::application::infra::{appsettings::MongoSettings, env};

pub async fn create_mongo_client(settings: &MongoSettings) -> Result<Client> {
    let password = env::get("MONGO_PASSWORD")?;
    let connection_string = settings
        .connection_string
        .replace("{USER}", &settings.user)
        .replace("{PASSWORD}", &password)
        .replace("{URL}", &settings.url);

    let options = ClientOptions::parse(connection_string).await?;
    Ok(Client::with_options(options)?)
}
