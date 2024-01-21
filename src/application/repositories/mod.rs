use anyhow::Error;
use tokio_postgres::{Client, NoTls};

use super::infra::appsettings::PostgresSettings;

pub mod command;
pub mod guild;
pub mod jukebox;
pub mod stats;
pub mod user;

pub async fn ensure_database_created(client: &Client) -> Result<(), Error> {
    client
        .batch_execute(include_str!("queries/create.sql"))
        .await?;

    Ok(())
}

pub async fn get_client(settings: &PostgresSettings) -> Result<tokio_postgres::Client, Error> {
    let (client, connection) =
        tokio_postgres::connect(&settings.get_connection_string()?, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            log::error!("connection error: {}", e);
        }
    });

    Ok(client)
}
