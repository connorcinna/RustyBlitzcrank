use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::{CommandDataOption, CommandDataOptionValue};

use std::collections::HashMap;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE, ACCEPT};
use reqwest::Client;
use regex::Regex;

extern crate serde_json;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

pub async fn run(options: &[CommandDataOption]) -> String {
    dotenv().ok();
    let option = options
            .get(0)
            .expect("Expected query option")
            .resolved
            .as_ref()
            .expect("Expected query option");
    if let CommandDataOptionValue::String(query) = option {
        let api_key = env::var("GPT_KEY").unwrap();
        let endpoint_url = "https://api.openai.com/v1/chat/completions";
        let mut headers = HeaderMap::new();
        let authorization_header = format!("Bearer {}", api_key);
        headers.insert(AUTHORIZATION, authorization_header.parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(ACCEPT,"application/json".parse().unwrap());

        let req_body = serde_json::json!({"prompt": query, "model" : "gpt-3.5-turbo"});
        println!("{}", req_body.to_string());
        let client = Client::new();

        let resp = client.post(endpoint_url)
            .headers(headers)
            .json::<serde_json::Value>(&req_body)
            .send()
            .await
            .unwrap();
        println!("{:?}", resp);

        if resp.status().is_success() {
            println!("Success response");
            //let blacklist = Regex::new(r"\n+").unwrap();
            let mut resp_str = resp.json::<serde_json::Value>()
                .await
                .unwrap()
                .get("choices")
                .unwrap()[0]
                //.get("text")
                //.unwrap()
                .to_string();
                //.replace("\\n", "");
            if resp_str.is_empty() {
                return String::from("No response: try a longer or more detailed prompt");
            }
            //resp_str.push_str("\n");
            println!("{}", resp_str);
            return resp_str;
        } 
        else {
            println!("Error response");
            //println!("Error: {:?}", resp.status());
            let resp_error = resp.text().await.unwrap();
            println!("Error: {:?}", resp_error);
            return resp_error; 
        }
    } 
    else {
        return String::from("how did this happen");
    }

}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("gpt").description("Query ChatGPT with a prompt")
    .create_option(|option| {
        option
            .name("prompt")
            .description("The prompt to be passed to ChatGPT")
            .kind(CommandOptionType::String)
            .required(true)
        });
    return command;
}
