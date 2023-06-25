#[cfg(test)]
mod deploy {
    use anyhow::Error;
    use lib::{application::infra::http_clients::github_client::GithubClient, get_settings};
    use reqwest::Client;

    #[tokio::test]
    async fn should_trigger_deploy() -> Result<(), Error> {
        let settings = get_settings()?;

        let _client = GithubClient::new(Client::new(), settings.github_settings);

        //client.deploy().await?;

        Ok(())
    }
}
