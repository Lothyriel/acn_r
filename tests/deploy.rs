#[cfg(test)]
mod deploy {
    use acn_r::{application::infra::http_clients::github_client::GithubClient, init_app};
    use anyhow::Error;
    use reqwest::Client;

    #[tokio::test]
    async fn should_trigger_deploy() -> Result<(), Error> {
        let settings = init_app()?;

        let _client = GithubClient::new(Client::new(), settings.github_settings);

        //client.deploy().await?;

        Ok(())
    }
}
