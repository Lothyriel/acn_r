use anyhow::{anyhow, Error};
use reqwest::Client;
use serde_json::Value;

const URL: &str = "https://api.rule34.xxx/index.php?page=dapi&s=post&q=index&json=1";
const LAST: &str = "&limit=1";

pub struct R34Client {
    client: Client,
}

impl R34Client {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn random(&self, prompt: Option<String>) -> Result<&str, Error> {

        match prompt {
            Some(_) => { self.tag_spam().await },
            None => { self.random_spam().await },
        }
        
    }

    pub async fn random_spam(&self) -> Result<&str, Error> {

        let response = self.client.get(format!("{URL}{LAST}")).send().await?;
        let response = response.json::<Value>().await?;
        let last_result = response
            .as_array()
            .ok_or_else(|| anyhow!("JSON is not an array!"))?;

        let last_result = last_result[0]
            .as_object()
            .ok_or_else(|| anyhow!("JSON is not an object!"))?;

        let last_id = last_result
            .get("id")
            .ok_or_else(|| anyhow!("Object doesn't have this key!"))?;

        println!("{last_id}");

        todo!()
    }

    async fn tag_spam(&self) -> Result<&str, Error> {
        todo!()
    }
}
