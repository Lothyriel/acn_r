#[cfg(test)]
mod stats {
    use std::str::FromStr;

    use acn_r::{
        application::{
            dependency_configuration::DependencyContainer,
            models::{dto::user::UpdateActivityDto, entities::user::Activity},
            services::{user_services::UserServices, stats_services::OnlineStatusProvider},
        },
        init_app,
    };
    use anyhow::{anyhow, Error};
    use chrono::Duration;
    use mongodb::bson::oid::ObjectId;
    use serenity::async_trait;

    const LA_PALOMBA_ID: u64 = 244922266050232321;
    const LOTHYRIEL_ID: u64 = 244922703667003392;
    const SECONDS_IN_10_HOURS: i64 = 60 * 60 * 10;

    #[tokio::test]
    async fn should_get_stats() -> Result<(), Error> {
        let settings = init_app()?;
        let data = DependencyContainer::build(settings).await?;

        populate_test_stats(data.user_services).await?;

        let guild_stats = data
            .stats_services
            .get_guild_stats(LA_PALOMBA_ID, Some(LOTHYRIEL_ID))
            .await?;

        let lothyriel_data = guild_stats
            .stats
            .iter()
            .find(|e| e.user_id == LOTHYRIEL_ID)
            .ok_or_else(|| anyhow!("Couldn't find this user's data"))?;

        let spent_some_time = lothyriel_data.seconds_online >= SECONDS_IN_10_HOURS;
        assert!(spent_some_time);
        assert_eq!(lothyriel_data.user_id, LOTHYRIEL_ID);

        Ok(())
    }
    
    #[tokio::test]
    async fn should_filter_activity_discrepancies() -> Result<(), Error> {
        let settings = init_app()?;
        let data = DependencyContainer::build(settings).await?;

        let spoiled_id = ObjectId::from_str("647fbb6bba0e327540cfed7b")?;

        let removed_ids = data
            .stats_services
            .clean_spoiled_stats(LA_PALOMBA_ID, MockStatusProvider)
            .await?;

        assert!(removed_ids[0] == spoiled_id);

        Ok(())
    }

    struct MockStatusProvider;

    #[async_trait]
    impl OnlineStatusProvider for MockStatusProvider {
        async fn get_status(&self) -> Result<Vec<u64>, Error> {
            Ok(vec![LOTHYRIEL_ID])
        }
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
