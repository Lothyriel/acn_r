#[cfg(test)]
mod deploy {
    use anyhow::Error;
    use lib::application::infra::{appsettings, http_clients::github_client::GithubClient};
    use reqwest::Client;

    #[tokio::test]
    async fn should_trigger_deploy() -> Result<(), Error> {
        let settings = appsettings::load()?;

        let client = GithubClient::new(Client::new(), settings.github_settings);

        client.deploy().await?;

        Ok(())
    }
}
