use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::{CommandDataOption, CommandDataOptionValue};

extern crate dotenv;
extern crate serde_json;
use dotenv::dotenv;
use serde_json::Value;
use reqwest;


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
        let url = format!("https://www.googleapis.com/customsearch/v1?key=AIzaSyCDvi2YxuEsz5uxR1e1h6gq2iF9Ly_WPZU&cx=71446e05228ee4314&q={}&searchType=image&fileType=jpg&alt=json", query);
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
            .and_then(|value| value.get("link"))
            .unwrap()
            .to_string();
        format!("Searching for image: {}\n{}", query, &result[1..result.len()-1])
    }
    else {
        String::from("Fuck")
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