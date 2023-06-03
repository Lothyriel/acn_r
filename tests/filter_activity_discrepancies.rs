#[cfg(test)]
mod filter_activity_discrepancies {
    use acn_r::{
        application::{
            dependency_configuration::DependencyContainer, models::entities::user::Activity,
        },
        init_app,
    };
    use anyhow::Error;

    const LA_PALOMBA_ID: u64 = 244922266050232321;
    const LOTHYRIEL_ID: u64 = 244922703667003392;

    #[tokio::test]
    async fn should_filter_activity_discrepancies() -> Result<(), Error> {
        let settings = init_app()?;
        let data = DependencyContainer::build(settings).await?;

        let activities = data
            .stats_services
            .get_activities(LA_PALOMBA_ID, Some(LOTHYRIEL_ID))
            .await?;

        let users_online = vec![LOTHYRIEL_ID];

        let users_with_discrepancies: Vec<_> = activities
            .iter()
            .filter(|act| {
                let (connects, disconnects): (Vec<_>, Vec<_>) = act
                    .1
                    .iter()
                    .partition(|a| a.activity_type == Activity::Connected);

                match connects.len() - disconnects.iter().len() {
                    0 => false,
                    1 if users_online.contains(&act.0) => false,
                    _ => true,
                }
            })
            .collect();

        //let ids_to_delete = users_with_discrepancies.into_iter().map

        Ok(())
    }
}
