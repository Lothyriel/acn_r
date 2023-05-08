use crate::application::{infra::env_var, models::appsettings::AppSettings};
use anyhow::Error;
use mongodb::{options::ClientOptions, Client};

pub async fn create_mongo_client(appsettings: &AppSettings) -> Result<Client, Error> {
    let password = env_var::get("MONGO_PASSWORD")?;
    let connection_string = appsettings
        .mongo_connection_string
        .replace("{USER}", &appsettings.mongo_user)
        .replace("{PASSWORD}", &password)
        .replace("{CLUSTER_URL}", &appsettings.mongo_cluster_url);

    let options = ClientOptions::parse(connection_string).await?;
    Ok(Client::with_options(options)?)
}
