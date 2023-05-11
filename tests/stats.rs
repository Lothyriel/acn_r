#[cfg(test)]
mod stats {
    use acn_r::{
        application::{
            dependency_configuration::DependencyContainer,
            models::{dto::user::UpdateActivityDto, entities::user::Activity},
            services::user_services::UserServices,
        },
        init_app,
    };
    use anyhow::{anyhow, Error};
    use chrono::Duration;

    const LA_PALOMBA_ID: u64 = 244922266050232321;
    const LOTHYRIEL_ID: u64 = 244922703667003392;

    #[tokio::test]
    async fn should_get_stats() -> Result<(), Error> {
        let settings = init_app()?;
        let data = DependencyContainer::build(settings).await?;

        populate_test_stats(data.user_services).await?;

        let guild_stats = data
            .stats_services
            .get_stats_of_guild(LA_PALOMBA_ID, Some(LOTHYRIEL_ID))
            .await?;

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

    async fn populate_test_stats(user_services: UserServices) -> Result<(), Error> {
        let mut date = chrono::Utc::now();

        for _ in 0..10 {
            let connected = UpdateActivityDto {
                user_id: LOTHYRIEL_ID,
                guild_id: LA_PALOMBA_ID,
                guild_name: "La Palombert".to_owned(),
                nickname: "Lothyriel".to_owned(),
                activity: Activity::Connected,
                date,
            };
            user_services.update_user_activity(connected).await?;

            date = date + Duration::hours(1);

            let disconnected = UpdateActivityDto {
                user_id: LOTHYRIEL_ID,
                guild_id: LA_PALOMBA_ID,
                guild_name: "La Palombert".to_owned(),
                nickname: "Lothyriel".to_owned(),
                activity: Activity::Disconnected,
                date,
            };
            user_services.update_user_activity(disconnected).await?;

            date = date + Duration::hours(1);
        }

        Ok(())
    }
}
