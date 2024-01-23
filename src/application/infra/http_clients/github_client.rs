use anyhow::Error;
use reqwest::Client;

use crate::application::infra::{appsettings::GithubSettings, env};

const URL: &str = "https://api.github.com/repos/{USER}/{REPO}/actions/workflows/{FILE}/dispatches";

pub struct GithubClient {
    client: Client,
    settings: GithubSettings,
}

impl GithubClient {
    pub fn new(client: Client, settings: GithubSettings) -> Self {
        Self { client, settings }
    }

    pub async fn deploy(&self) -> Result<(), Error> {
        let pat = env::get("GITHUB_PAT")?;
        let url = URL
            .replace("{USER}", &self.settings.username)
            .replace("{REPO}", &self.settings.repository)
            .replace("{FILE}", "acn.yml");

        let body = serde_json::json!({
            "ref": self.settings.branch_name,
        });

        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", pat))
            .header("User-Agent", &self.settings.username)
            .json(&body)
            .send()
            .await?;

        response.error_for_status()?;

        Ok(())
    }
}
