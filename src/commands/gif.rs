extern crate dotenv;
extern crate serde_json;
use dotenv::dotenv;
use serde_json::Value;
use std::env;
use reqwest;
use crate::{Context, Error};

#[poise::command(slash_command, rename = "gif")]
pub async fn run
(
    ctx: Context<'_>,
    #[description = "The query to be passed to Tenor's API"] query: String
) -> Result<(), Error>
{
    dotenv().ok();
    let query_string: String = format!("https://g.tenor.com/v1/search?q={}&key={}&limit=1", query, env::var("TENOR_KEY").expect("Expected a Tenor key"));
    let result_code = reqwest::get(query_string.clone())
        .await
        .unwrap()
        .status();
    match result_code {
        reqwest::StatusCode::OK => {
            let reqwest_result = reqwest::get(query_string.clone())
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            let json_result: Value = serde_json::from_str(&reqwest_result).unwrap();
            let url = json_result.get("results")
                .and_then(|value| value.get(0))
                .and_then(|value| value.get("url"))
                .unwrap()
                .to_string();
            let url = &url[1..url.len()-1];
            let _ = ctx.say(format!("\n{}", url)).await;
            Ok(())
        },
        reqwest::StatusCode::UNAUTHORIZED => {
            let _ = ctx.say("Error authorizing request").await;
            Err(Error::from("Error authorizing request"))
        }
        _ => {
            panic!("Unexpected error in Tenor response");
        }
    }
}
