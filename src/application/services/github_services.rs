use anyhow::Error;
use log::info;
use mongodb::Database;

#[derive(Clone)]
pub struct GithubServices {}

impl GithubServices {
    pub fn new(_database: &Database) -> Self {
        Self {}
    }

    pub async fn start_deploy(&self) -> Result<(), Error> {
        info!("(TODO!) Should call Github API and trigger the action deploy");
        Ok(())
    }
}
