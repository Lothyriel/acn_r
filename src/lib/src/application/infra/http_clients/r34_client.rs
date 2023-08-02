use anyhow::{anyhow, Error};
use futures::future::join_all;
use poise::serenity_prelude::MessageBuilder;
use rand::prelude::*;
use reqwest::Client;
use serde_json::Value;

const URL: &str = "https://api.rule34.xxx/index.php?page=dapi&s=post&q=index&json=1";
const LAST: &str = "&limit=1";
const POST_ID: &str = "&id=";

pub struct R34Client {
    client: Client,
}

impl R34Client {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn random(&self, prompt: Option<String>) -> Result<String, Error> {
        match prompt {
            Some(_) => self.tag_spam().await,
            None => self.random_spam().await,
        }
    }

    async fn random_spam(&self) -> Result<String, Error> {
        let last_id = self.get_last_id().await?;        

        let get_url_tasks = (0..4).into_iter().map(|_| self.get_url(last_id));

        let urls = join_all(get_url_tasks).await;

        let mut message_builder = MessageBuilder::new();

        for url_result in urls {
            match url_result {
                Ok(url) => { message_builder.push_line(url); },
                Err(_) => {},
            }
        }

        Ok(message_builder.build())
    }

    async fn get_url(&self, last_id: u64) -> Result<String, Error> {
        let response = self.client.get(format!("{URL}{POST_ID}{}", get_rand(last_id))).send().await?;
        let response = response.json::<Value>().await?;
        let result = response.as_array().ok_or_else(|| anyhow!("JSON is not an array!"))?;
        let result = result[0].as_object().ok_or_else(|| anyhow!("JSON is not an object!"))?;
        let result_id = result
        .get("file_url")
        .ok_or_else(|| anyhow!("Object doesn't have this key!"))?
        .as_str()
        .ok_or_else(|| anyhow!("Not a valid url!"))?;
        Ok(result_id.to_owned())
    }

    async fn get_last_id(&self) -> Result<u64, Error> {        
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
            .ok_or_else(|| anyhow!("Object doesn't have this key!"))?
            .as_u64()
            .ok_or_else(|| anyhow!("Not a valid number!"))?;

        Ok(last_id)
    }

    async fn tag_spam(&self) -> Result<String, Error> {
        todo!()
    }
}

fn get_rand(last_id: u64) -> u64 {
    thread_rng().gen_range(0..last_id)
}
