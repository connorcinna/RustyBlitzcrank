extern crate dotenv;
extern crate serde_json;
use dotenv::dotenv;
use serde_json::Value;
use reqwest;
use std::env;
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn run
(
    ctx: Context<'_>,
    #[description = "The query to be passed to YouTube's API"] query: String
) -> Result<(), Error>
{
    dotenv().ok();
    let client = reqwest::Client::new();
    //TODO: build the url with params in reqest and not stuffing the whole URL here
    let url = format!("https://youtube.googleapis.com/youtube/v3/search?q={}&safeSearch=safeSearchSettingUnspecified&videoEmbeddable=videoEmbeddableUnspecified&key={}", query, env::var("YT_KEY").unwrap());
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
    println!("{}", result);
    let _ = ctx.say(format!("\nhttps://www.youtube.com/watch?v={}", &result[1..result.len()-1])).await;
    Ok(())
}
