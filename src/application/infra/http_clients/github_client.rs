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
            .replace("{FILE}", &self.settings.workflow_file);

        let body = serde_json::json!({
            "ref": self.settings.branch_name,
        });

        let response = self
            .client
            .post(url)
            .header("Accept", "application/vnd.github.v3+json")
            .header("Authorization", format!("Bearer {}", pat))
            .json(&body)
            .send()
            .await?;

        response.error_for_status()?;

        Ok(())
    }
}
