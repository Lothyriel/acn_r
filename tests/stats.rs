#[cfg(test)]
mod stats {
    use std::collections::HashSet;

    use acn_r::{
        application::{
            dependency_configuration::RepositoriesContainer,
            infra::status_monitor::StatusManager,
            models::{dto::user::UpdateActivityDto, entities::user::Activity},
            repositories::{guild::GuildRepository, stats::StatsRepository, user::UserRepository},
        },
        extensions::{serenity::guild_ext::StatusInfo, std_ext},
        init_app,
    };
    use anyhow::{anyhow, Error};
    use chrono::{Days, Duration};
    use mongodb::Database;

    const LA_PALOMBA_ID: u64 = 244922266050232321;
    const LOTHYRIEL_ID: u64 = 244922703667003392;
    const SECONDS_IN_10_HOURS: i64 = 60 * 60 * 10;

    #[tokio::test]
    async fn should_get_stats() -> Result<(), Error> {
        let settings = init_app()?;
        let db = RepositoriesContainer::database(&settings).await?;
        let stats_repository = StatsRepository::new(&db);

        populate_test_stats(&db).await?;

        let guild_stats = stats_repository
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

    #[test]
    fn should_get_difference() -> Result<(), Error> {
        let jx = StatusInfo::new(1, 1);
        let junior = StatusInfo::new(2, 1);

        let mut current_online = HashSet::new();
        current_online.insert(jx);

        let mut manager = StatusManager::new(current_online);

        let mut new_online = HashSet::new();
        new_online.insert(junior);

        let update = manager.update_status(new_online);

        assert!(update[0].activity_type == Activity::Connected);
        assert!(update[0].user_id == 2);

        assert!(update[1].activity_type == Activity::Disconnected);
        assert!(update[1].user_id == 1);

        Ok(())
    }

    async fn populate_test_stats(db: &Database) -> Result<(), Error> {
        let user_repository = UserRepository::new(&db, GuildRepository::new(&db));

        let mut date = chrono::Utc::now().checked_sub_days(Days::new(2)).unwrap();

        for _ in 0..10 {
            let connected = UpdateActivityDto {
                user_id: LOTHYRIEL_ID,
                guild_id: LA_PALOMBA_ID,
                guild_name: "La Palombert".to_owned(),
                nickname: "Lothyriel".to_owned(),
                activity: Activity::Connected,
                date,
            };
            user_repository.update_user_activity(connected).await?;

            date = date + Duration::hours(1);

            let disconnected = UpdateActivityDto {
                user_id: LOTHYRIEL_ID,
                guild_id: LA_PALOMBA_ID,
                guild_name: "La Palombert".to_owned(),
                nickname: "Lothyriel".to_owned(),
                activity: Activity::Disconnected,
                date,
            };

            user_repository.update_user_activity(disconnected).await?;

            date = date + Duration::hours(1);
        }

        Ok(())
    }
}
