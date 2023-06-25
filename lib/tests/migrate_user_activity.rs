#[cfg(test)]
mod migrate {
    use acn_lib::{
        application::{
            infra::mongo_client::create_mongo_client,
            models::entities::{user::Activity, user_activity::UserActivity},
        },
        get_test_settings,
    };
    use anyhow::Error;
    use chrono::Utc;
    use serde::Deserialize;

    #[tokio::test]
    async fn should_migrate_user_activity() -> Result<(), Error> {
        let settings = get_test_settings()?;

        let collection = create_mongo_client(&settings.mongo_settings)
            .await?
            .database("acn_r")
            .collection::<UserActivity>("UserActivity");

        let data = get_data().await?;

        collection.insert_many(data, None).await?;

        Ok(())
    }

    async fn get_data() -> Result<Vec<UserActivity>, Error> {
        let path = "./acn_r.UserActivity.json";

        let settings_path = std::fs::read_to_string(path)?;

        std::fs::remove_file(path)?;

        #[derive(Deserialize)]
        struct Long {
            #[serde(rename = "$numberLong")]
            number_long: String,
        }

        #[derive(Deserialize)]
        struct TempActivity {
            guild_id: Long,
            user_id: Long,
            date: chrono::DateTime<Utc>,
            activity_type: Activity,
        }

        let activities: Vec<TempActivity> = serde_json::from_str(&settings_path)?;

        Ok(activities
            .into_iter()
            .map(|a| UserActivity {
                guild_id: a.guild_id.number_long.parse().unwrap(),
                user_id: a.user_id.number_long.parse().unwrap(),
                date: a.date,
                activity_type: a.activity_type,
            })
            .collect())
    }
}
