use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::{CommandDataOption, CommandDataOptionValue};

use std::collections::HashMap;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE, ACCEPT};
use reqwest::Client;

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
        //let endpoint_url = "https://api.chatgpt.com/ask";
        //TODO check if key is actually correct
        let mut headers = HeaderMap::new();
        let authorization_header = format!("Bearer {}", api_key);
        headers.insert(AUTHORIZATION, authorization_header.parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(ACCEPT,"application/json".parse().unwrap());
        for (key, val) in &headers {
            println!("{:?} : {:?}", key, val);
        }

        let mut req_body = HashMap::new();
        req_body.insert("prompt", query);
        println!("req_body");
        for (key, value) in &req_body {
            println!("{} : {}", key, value);
        }
        let client = Client::new();

        let resp = client.post(endpoint_url)
            .headers(headers)
            .json(&req_body)
            .send()
            .await
            .unwrap();
        println!("{:?}", resp);

        if resp.status().is_success() {
            println!("Success response");
            let resp_body: HashMap<String, String> = resp.json().await.unwrap();
            let response = resp_body.get("response").unwrap();

            println!("{}", response);
            return response.to_owned();
        } 
        else {
            println!("Error response");
            println!("Error: {:?}", resp.status());
            return resp.status().to_string(); 
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
