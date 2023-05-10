use crate::application::{infra::env};
use anyhow::Error;
use mongodb::{options::ClientOptions, Client};

use super::appsettings::AppSettings;

pub async fn create_mongo_client(appsettings: &AppSettings) -> Result<Client, Error> {
    let password = env::get("MONGO_PASSWORD")?;
    let connection_string = appsettings
        .mongo_connection_string
        .replace("{USER}", &appsettings.mongo_user)
        .replace("{PASSWORD}", &password)
        .replace("{URL}", &appsettings.mongo_url);

    let options = ClientOptions::parse(connection_string).await?;
    Ok(Client::with_options(options)?)
}
