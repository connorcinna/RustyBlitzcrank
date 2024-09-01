use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::{CommandDataOption, CommandDataOptionValue};
#[allow(deprecated)]
use serenity::model::interactions::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::prelude::*;


extern crate dotenv;
extern crate serde_json;
use dotenv::dotenv;
use serde_json::Value;
use reqwest;


pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send>>;

pub async fn run(options: &[CommandDataOption]) -> Result<String> {
    dotenv().ok();
    let option = options
            .get(0)
            .expect("Expected query option")
            .resolved
            .as_ref()
            .expect("Expected query option");
    if let CommandDataOptionValue::String(query) = option {
        let client = reqwest::Client::new();
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
        Ok(format!("\n{}", &result[1..result.len()-1]))
    }
    //dont think this is ever reached?
    else {
        return Err(Box::new(std::fmt::Error));
    }
}


pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("search").description("Search Google images")
    .create_option(|option| {
        option
            .name("word")
            .description("The query to be passed to Google's search API")
            .kind(CommandOptionType::String)
            .required(true)
        });
    return command;
}

//handle deferring the message, wait for the response from API call, and send it to the channel
//or, if the API didn't find an image link, send the funny blitzcrank picture
#[allow(deprecated)]
pub async fn interaction(ctx: &Context, command: &ApplicationCommandInteraction) -> bool
{
    command.create_interaction_response(&ctx.http, |response| 
    {
        response
            .kind(InteractionResponseType::DeferredChannelMessageWithSource)
            .interaction_response_data(|message| message.content(command.data.name.as_str()))
    }).await.unwrap();
    if let Ok(res) = run(&command.data.options).await 
    {
        command.edit_original_interaction_response(&ctx.http, |response| response.content(res)).await.unwrap();
        return true;
    }
    else 
    {
        return false;
    }
}
