extern crate dotenv;
extern crate serde_json;
use std::env;
use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use serenity::builder::{CreateAttachment, CreateMessage};
use serde_json::Value;
use std::collections::HashMap;
use reqwest;
use tokio::fs::File;
use crate::{Error, Context};

fn build_url(query: String) -> Result<reqwest::Url, Error>
{
    let key = env::var("GOOGLE_PUBLIC_KEY").expect("Did not find GOOGLE_PUBLIC_KEY in environment");
    let cx = env::var("GOOGLE_CX").expect("Did not find GOOGLE_CX in environment");
    let mut params = HashMap::new();
    params.insert("key", key.as_str());
    params.insert("cx", cx.as_str());
    params.insert("q", query.as_str());
    params.insert("searchType", "image");
    params.insert("fileType", "jpg");
    params.insert("alt", "json");
    params.insert("num", "1");
    Ok(reqwest::Url::parse_with_params("https://www.googleapis.com/customsearch/v1?", params.clone()).expect("Unable to parse URL"))
}

#[poise::command(slash_command, rename = "search")]
pub async fn run(
    ctx: Context<'_>,
    #[description = "The query to pass to Google's search API"] query: String
) -> Result<(), Error>
{
    let _ = ctx.defer().await;
    dotenv().ok();
    let url = build_url(query).unwrap();
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let json_result: Value = serde_json::from_str(&response).unwrap();
    println!("{}", serde_json::to_string_pretty(&json_result).unwrap());
    match json_result.get("queries")
        .and_then(|value| value.get("nextPage"))
        .and_then(|value| value.get(0))
        .and_then(|value| value.get("totalResults"))
        {
            Some(num_results) =>
            {
                if num_results == "0"
                {
                    no_results(ctx).await?
                }
            }
            None => {
                no_results(ctx).await?
            }
    }
    let result = json_result.get("items")
        .and_then(|value| value.get(0))
        .and_then(|value| value.get("link"))
        .unwrap()
        .to_string();
    let _ = ctx.say(format!("\n{}", &result[1..result.len()-1])).await;
    Ok(())
}

async fn no_results(ctx: Context<'_>) -> Result<(), Error>
{
    let channel_id = ctx.channel_id();
    let mut img_path = std::env::current_dir().unwrap();
    img_path.push("resources/lol.png");
    let img_file = File::open(img_path).await.unwrap();
    let files =
    [
        CreateAttachment::file(&img_file, "lol.png").await?,
    ];
    let builder = CreateMessage::new().content("");
    let _ = channel_id.send_files(&ctx.http(), files, builder).await;
    Err("No results found".into())
}
