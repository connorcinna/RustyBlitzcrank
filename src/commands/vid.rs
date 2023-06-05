use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::{CommandDataOption, CommandDataOptionValue};

extern crate dotenv;
extern crate serde_json;
use dotenv::dotenv;
use serde_json::Value;
use reqwest;
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
        let client = reqwest::Client::new();
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
        format!("\nhttps://www.youtube.com/watch?v={}", &result[1..result.len()-1])
    }
    else {
        String::from("Fuck")
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("vid").description("Search YouTube for a video")
    .create_option(|option| {
        option
            .name("word")
            .description("The query to be passed to YouTube's search API")
            .kind(CommandOptionType::String)
            .required(true)
        });
    return command;
}