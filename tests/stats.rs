#[cfg(test)]
mod stats {
    use acn_r::{
        application::{
            dependency_configuration::DependencyContainer,
            models::{dto::user::UpdateActivityDto, entities::user::Activity},
            services::{
                guild_services::GuildServices,
                stats_services::{OnlineStatusProvider, StatsServices},
                user_services::UserServices,
            },
        },
        init_app,
    };
    use anyhow::{anyhow, Error};
    use chrono::Duration;
    use mongodb::{bson::oid::ObjectId, Database};
    use poise::async_trait;

    const LA_PALOMBA_ID: u64 = 244922266050232321;
    const LOTHYRIEL_ID: u64 = 244922703667003392;
    const SECONDS_IN_10_HOURS: i64 = 60 * 60 * 10;

    #[tokio::test]
    async fn should_get_stats() -> Result<(), Error> {
        let settings = init_app()?;
        let db = DependencyContainer::database(&settings).await?;
        let stats_services = StatsServices::new(&db);

        populate_good_test_stats(&db).await?;

        let guild_stats = stats_services
            .get_guild_stats(LA_PALOMBA_ID, Some(LOTHYRIEL_ID), MockStatusProvider)
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
        let db = DependencyContainer::database(&settings).await?;

        let bad_activity = populate_bad_test_stats(&db).await?;

        let stats_services = StatsServices::new(&db);

        let removed_ids = stats_services
            .clean_spoiled_stats(LA_PALOMBA_ID, MockStatusProvider)
            .await?;

        assert!(removed_ids.contains(&bad_activity));

        Ok(())
    }

    struct MockStatusProvider;

    #[async_trait]
    impl OnlineStatusProvider for MockStatusProvider {
        async fn get_status(&self) -> Result<Vec<u64>, Error> {
            Ok(vec![])
        }
    }

    async fn populate_good_test_stats(db: &Database) -> Result<(), Error> {
        let user_services = UserServices::new(&db, GuildServices::new(&db));

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

    async fn populate_bad_test_stats(db: &Database) -> Result<ObjectId, Error> {
        let user_services = UserServices::new(&db, GuildServices::new(&db));

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

        let wrong_disconnected = UpdateActivityDto {
            user_id: LOTHYRIEL_ID,
            guild_id: LA_PALOMBA_ID,
            guild_name: "La Palombert".to_owned(),
            nickname: "Lothyriel".to_owned(),
            activity: Activity::Disconnected,
            date,
        };

        let activity_id = user_services
            .update_user_activity(wrong_disconnected)
            .await?;

        let connected = UpdateActivityDto {
            user_id: LOTHYRIEL_ID,
            guild_id: LA_PALOMBA_ID,
            guild_name: "La Palombert".to_owned(),
            nickname: "Lothyriel".to_owned(),
            activity: Activity::Connected,
            date,
        };
        user_services.update_user_activity(connected).await?;

        Ok(activity_id)
    }
}
