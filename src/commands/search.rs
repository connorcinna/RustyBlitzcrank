extern crate dotenv;
extern crate serde_json;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{CommandInteraction, CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::builder::{CreateAttachment, CreateMessage};
use dotenv::dotenv;
use serde_json::Value;
use reqwest;
use tokio::fs::File;

use crate::{Error, Context};

//pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send>>;

#[poise::command(slash_command)]
pub async fn run(
    ctx: Context<'_>,
    #[description = "The query to pass to Google's search API"] query: String
) -> Result<(), Error>
{
    ctx.defer();
    dotenv().ok();
    let client = reqwest::Client::new();
    //TODO: reformat this with reqwest params instead of stuffing it all in this url string
    //also i just realized the key is exposed ?!
    let url = format!("https://www.googleapis.com/customsearch/v1?key=AIzaSyCDvi2YxuEsz5uxR1e1h6gq2iF9Ly_WPZU&cx=71446e05228ee4314&q={}&searchType=image&fileType=jpg&alt=json&num=1", query);
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
            Some(result_num) =>
            {
                if result_num == "0"
                {
                    no_results(ctx);
                    return Err(Box::new(std::fmt::Error));
                }
            }
            None => {
                return Err(Box::new(std::fmt::Error));
            }
    }
    let result = json_result.get("items")
        .and_then(|value| value.get(0))
        .and_then(|value| value.get("link"))
        .unwrap()
        .to_string();

    //right now assuming that ctx.say() will foil the ctx.defer() from earlier
    ctx.say(format!("\n{}", &result[1..result.len()-1]));
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
    //empty message closure to satisfy function
    let builder = CreateMessage::new().content("");
    let _ = channel_id.send_files(&ctx.http(), files, builder).await;
    Ok(())
//    //get rid of the "bot is thinking..." message
//    command.delete_original_interaction_response(&ctx.http).await.unwrap();
}
