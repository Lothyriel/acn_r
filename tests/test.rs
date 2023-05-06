#[cfg(test)]
mod tests {
    use acn_r::application::{
        infra::mongo_client::create_mongo_client,
        services::{appsettings_service, stats_services::StatsServices},
    };
    use anyhow::{anyhow, Error};
    use chrono::Duration;
    use mongodb::Database;

    #[tokio::test]
    async fn should_get_stats() -> Result<(), Error> {
        let db = get_database().await?;
        let services = StatsServices::new(&db);
        const LA_PALOMBA_ID: u64 = 244922266050232321;
        const LOTHYRIEL_ID: u64 = 244922703667003392;

        let guild_stats = services.get_stats_of_guild(LA_PALOMBA_ID).await?;

        let lothyriel_data = guild_stats
            .into_iter()
            .find(|e| e.0 == LOTHYRIEL_ID)
            .ok_or_else(|| anyhow!("Não encontrado"))?;

        let test = lothyriel_data.1 > Duration::seconds(1000);
        assert_eq!(lothyriel_data.0, LOTHYRIEL_ID);
        assert!(test);

        Ok(())
    }

    async fn get_database() -> Result<Database, Error> {
        dotenv::dotenv().ok();
        let settings = appsettings_service::load()?;
        Ok(create_mongo_client(&settings).await?.database("acn_r"))
    }
}
