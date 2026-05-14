use anyhow::anyhow;
use log::error;
use poise::{
    command,
    serenity_prelude::{CreateMessage, User},
};
use rand::RngExt;
use serde::Deserialize;

use crate::{
    application::infra::env,
    extensions::serenity::{CommandResult, Context},
};

const RULE34_API: &str = "https://api.rule34.xxx/index.php";
const RANDOM_POST_ID_UPPER_BOUND: u32 = 4_555_950;

#[derive(Debug, Deserialize)]
struct Rule34Post {
    file_url: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Rule34ApiResponse {
    Posts(Vec<Rule34Post>),
    ErrorMessage(String),
}

enum HentaiResponse {
    Url(String),
    NotFound,
}

async fn fetch_posts(
    ctx: Context<'_>,
    params: &[(&str, String)],
) -> anyhow::Result<Vec<Rule34Post>> {
    let mut params = params.to_vec();
    params.push(("user_id", env::get("RULE34_USER_ID")?));
    params.push(("api_key", env::get("RULE34_API_KEY")?));

    let response = ctx
        .data()
        .http_client
        .get(RULE34_API)
        .query(&params)
        .send()
        .await?;

    let body = response.text().await?;

    if body.trim().is_empty() {
        return Ok(Vec::new());
    }

    match serde_json::from_str::<Rule34ApiResponse>(&body)
        .map_err(|error| anyhow!("failed to decode rule34 response: {}", error))?
    {
        Rule34ApiResponse::Posts(posts) => Ok(posts),
        Rule34ApiResponse::ErrorMessage(message) => Err(anyhow!("rule34 api error: {}", message)),
    }
}

async fn fetch_hentai_response(
    ctx: Context<'_>,
    search: Option<&str>,
) -> anyhow::Result<HentaiResponse> {
    let base_params = vec![
        ("page", "dapi".to_owned()),
        ("json", "1".to_owned()),
        ("s", "post".to_owned()),
        ("q", "index".to_owned()),
    ];

    let posts = match search {
        Some(search) if !search.trim().is_empty() => {
            let mut params = base_params.clone();
            params.push(("tags", search.trim().replace(' ', "_")));
            fetch_posts(ctx, &params).await
        }
        _ => {
            let mut params = base_params.clone();
            let post_id = rand::rng().random_range(1..RANDOM_POST_ID_UPPER_BOUND);
            params.push(("id", post_id.to_string()));
            fetch_posts(ctx, &params).await
        }
    };

    let posts = posts?;

    if posts.is_empty() {
        return Ok(HentaiResponse::NotFound);
    }

    let selected_post = match search {
        Some(search) if !search.trim().is_empty() => {
            let index = rand::rng().random_range(0..posts.len());
            &posts[index]
        }
        _ => &posts[0],
    };

    match selected_post.file_url.clone() {
        Some(url) if !url.trim().is_empty() => Ok(HentaiResponse::Url(url)),
        _ => Err(anyhow!("rule34 response did not contain a usable file_url")),
    }
}

pub(super) async fn send_hentai(
    ctx: Context<'_>,
    target: Option<&User>,
    search: Option<&str>,
) -> CommandResult {
    let response = fetch_hentai_response(ctx, search).await.map_err(|error| {
        error!(
            "R34 command failed | user_id={} target={} search={:?} error={}",
            ctx.author().id.get(),
            target.map(|user| user.id.get()).unwrap_or_default(),
            search,
            error
        );

        anyhow!("request error: {}", error)
    })?;

    match response {
        HentaiResponse::Url(url) => match target {
            Some(target) => {
                let channel = target.create_dm_channel(ctx.http()).await?;
                channel
                    .send_message(ctx.http(), CreateMessage::new().content(url))
                    .await?;
            }
            None => {
                ctx.say(url).await?;
            }
        },
        HentaiResponse::NotFound => {
            ctx.say("tag not found").await?;
        }
    }

    Ok(())
}

#[command(prefix_command, slash_command, category = "R34", user_cooldown = 3)]
pub async fn random(
    ctx: Context<'_>,
    #[description = "Prompt to search for"]
    #[rest]
    search: Option<String>,
) -> CommandResult {
    send_hentai(ctx, None, search.as_deref()).await
}
