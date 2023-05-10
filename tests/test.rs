#[cfg(test)]
mod tests {
    use acn_r::application::{
        infra::{mongo_client::create_mongo_client, env, appsettings},
        services::{stats_services::StatsServices},
    };
    use anyhow::{anyhow, Error};
    use mongodb::Database;

    #[tokio::test]
    async fn should_get_stats() -> Result<(), Error> {
        let db = get_database().await?;
        let services = StatsServices::new(&db);
        const LA_PALOMBA_ID: u64 = 244922266050232321;
        const LOTHYRIEL_ID: u64 = 244922703667003392;

        let guild_stats = services.get_stats_of_guild(LA_PALOMBA_ID, Some(LOTHYRIEL_ID)).await?;

        let lothyriel_data = guild_stats
            .stats
            .iter()
            .find(|e| e.user_id == LOTHYRIEL_ID)
            .ok_or_else(|| anyhow!("Couldn't find this user's data"))?;

        let spent_some_time = lothyriel_data.seconds_online > 1000;
        assert!(spent_some_time);
        assert_eq!(lothyriel_data.user_id, LOTHYRIEL_ID);

        Ok(())
    }

    async fn get_database() -> Result<Database, Error> {
        env::init()?;
        let settings = appsettings::load()?;
        Ok(create_mongo_client(&settings).await?.database("acn_r"))
    }
}
