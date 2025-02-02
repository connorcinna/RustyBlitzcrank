extern crate dotenv;
extern crate serde_json;
use dotenv::dotenv;
use serde_json::Value;
use reqwest;
use std::env;
use std::collections::HashMap;
use crate::{Context, Error};

fn build_url(query: String) -> Result<reqwest::Url, Error>
{
    let key = env::var("YT_KEY").expect("Did not find YT_KEY in environment");
    let mut params = HashMap::new();
    params.insert("q", query.as_str());
    params.insert("safeSearch", "safeSearchSettingUnspecified");
    params.insert("videoEmbeddable", "videoEmbeddableUnspecified");
    params.insert("key", key.as_str());
    Ok(reqwest::Url::parse_with_params("https://youtube.googleapis.com/youtube/v3/search?", params.clone()).expect("Unable to parse URL"))
}

#[poise::command(slash_command, rename = "vid")]
pub async fn run
(
    ctx: Context<'_>,
    #[description = "The query to be passed to YouTube's API"] query: String
) -> Result<(), Error>
{
    let _ = ctx.defer().await;
    dotenv().ok();
    let client = reqwest::Client::new();
    let url = build_url(query).unwrap();
    let response = client
        .get(url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let json_result: Value = serde_json::from_str(&response).unwrap();
    let result = json_result.get("items")
        .and_then(|value| value.get(0))
        .and_then(|value| value.get("id"))
        .and_then(|value| value.get("videoId"))
        .unwrap()
        .to_string();
    let _ = ctx.say(format!("\nhttps://www.youtube.com/watch?v={}", &result[1..result.len()-1])).await;
    Ok(())
}
