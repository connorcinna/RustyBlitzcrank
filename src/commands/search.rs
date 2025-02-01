extern crate dotenv;
extern crate serde_json;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{CommandInteraction, CreateInteractionResponse, CreateInteractionResponseMessage};
use dotenv::dotenv;
use serde_json::Value;
use reqwest;

use crate::{Error, Context};

//pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send>>;

#[poise::command(slash_command)]
pub async fn run(
    ctx: Context<'_>,
    #[description = "The query to pass to Google's search API"] query: String
) -> Result<(), Error>
{
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
        .and_then(|value| value.get("totalResults")) {
        Some(result_num) => {
            if result_num == "0" {
                return Err(Box::new(std::fmt::Error));
            }
            //else, normal case
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

    //want to Defer the current interaction until the method finishes and then edit_response
    //how do I know when a poise slash command has finished executing from the context of
    //serenity's interaction_create?
    ctx.say(format!("\n{}", &result[1..result.len()-1]));
    Ok(())
}

//handle deferring the message, wait for the response from API call, and send it to the channel
//or, if the API didn't find an image link, send the funny blitzcrank picture
pub async fn interaction(ctx: serenity::Context, command: CommandInteraction)
{
    let data = CreateInteractionResponseMessage::new().content(command.data.name.as_str());
    let builder = CreateInteractionResponse::Defer(data);
    command.create_response(&ctx.http, builder).await.unwrap();
    if let Ok(res) = run(&ctx, &command.data.options).await
    {
        command.edit_response(&ctx.http, |response| response.content(res)).await.unwrap();
    }
//    return false;
//    send blitzcrank picture

}
