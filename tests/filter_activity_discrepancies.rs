#[cfg(test)]
mod filter_activity_discrepancies {
    use std::str::FromStr;

    use acn_r::{
        application::{
            dependency_configuration::DependencyContainer,
            services::stats_services::OnlineStatusProvider,
        },
        init_app,
    };
    use anyhow::Error;
    use mongodb::bson::oid::ObjectId;
    use serenity::async_trait;

    const LA_PALOMBA_ID: u64 = 244922266050232321;
    const LOTHYRIEL_ID: u64 = 244922703667003392;

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
}
