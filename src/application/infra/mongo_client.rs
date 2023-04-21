use anyhow::{anyhow, Error};
use mongodb::{options::ClientOptions, Client};
use serenity::prelude::TypeMapKey;
use crate::{application::models::appsettings::AppSettings, get_token_bot};

pub async fn create_mongo_client(appsettings: &AppSettings) -> Result<Client, Error> {
    let password = get_token_bot()?;
    let connection_string =
        "mongodb+srv://{USER}:{PASSWORD}@{CLUSTER_URL}/?retryWrites=true&w=majority"
            .replace("{USER}", &appsettings.mongo_user)
            .replace("{PASSWORD}", &password)
            .replace("{CLUSTER_URL}", &appsettings.mongo_cluster_url);

    let options = ClientOptions::parse(connection_string).await?;
    Client::with_options(options).map_err(|e| anyhow!(e))
}

pub struct MongoClient;

impl TypeMapKey for MongoClient {
    type Value = Client;
}