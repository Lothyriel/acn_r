#[cfg(test)]
mod migrate {
    use acn_r::{
        application::{
            infra::{env, mongo_client::create_mongo_client},
            models::entities::user_activity::UserActivity,
        },
        init_app,
    };
    use anyhow::Error;
    use reqwest::Client;
    use serde::Deserialize;

    #[tokio::test]
    async fn should_migrate_user_activity() -> Result<(), Error> {
        let settings = init_app()?;

        let collection = create_mongo_client(&settings.mongo_settings)
            .await?
            .database("acn_r")
            .collection::<UserActivity>("UserActivity");

        let data = get_data().await?;

        collection.insert_many(data, None).await?;

        Ok(())
    }

    async fn get_data() -> Result<Vec<UserActivity>, Error> {
        let client = Client::new();

        let url = "https://sa-east-1.aws.data.mongodb-api.com/app/data-bsxri/endpoint/data/v1/action/find";

        let body = serde_json::json!({
            "collection":"UserActivity",
            "database":"acn_r",
            "dataSource":"acn-cluster",
            "limit": 50000,
            "filter": {"activity_type": {
                "$in": [
                  "Connected",
                  "Disconnected"
                ]
              }}
        });

        let response = client
            .post(url)
            .header("api-key", env::get("MIGRATION_API_KEY")?)
            .json(&body)
            .send()
            .await?;

        response.error_for_status_ref()?;

        #[derive(Deserialize)]
        struct Output {
            documents: Vec<UserActivity>,
        }

        let output: Output = response.json().await?;

        Ok(output.documents)
    }
}
