use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::{CommandDataOption, CommandDataOptionValue};

extern crate dotenv;
extern crate serde_json;
use dotenv::dotenv;
use serde_json::Value;
use serde_json::json;
use std::env;
use reqwest;

pub async fn run(options: &[CommandDataOption]) -> String {
    dotenv().ok();
    let option = options
        .get(0)
        .expect("Expected prompt option")
        .resolved
        .as_ref()
        .expect("Expected prompt option");
    if let CommandDataOptionValue::String(query) = option {
            let url: String = format!("https://generativelanguage.googleapis.com/v1beta3/models/text-bison-001:generateText?key={}", env::var("GOOGLE_PALM_KEY").expect("Expected a PaLM key"));
            let client = reqwest::Client::new();
            let res = client
                .post(url.clone())
                .json(&json!({
                    "prompt": {
                        "text": query 
                    },
                    //2edgy4me
                    "safety_settings": [
                        {"category": "HARM_CATEGORY_DEROGATORY", "threshold":4},
                        {"category": "HARM_CATEGORY_TOXICITY", "threshold":4},
                        {"category": "HARM_CATEGORY_VIOLENCE", "threshold":4},
                        {"category": "HARM_CATEGORY_SEXUAL", "threshold":4},
                        {"category": "HARM_CATEGORY_MEDICAL", "threshold":4},
                        {"category": "HARM_CATEGORY_DANGEROUS", "threshold":4},
                    ],
                    "temperature" : 1,
                    "max_output_tokens": 4096,
                }))
                .send()
                .await
                .unwrap();
            match res.status() {
                reqwest::StatusCode::OK => {
                    let res_text = res.text().await.unwrap();
                    let json_result: Value = serde_json::from_str(&res_text).unwrap();
                    let output = json_result.get("candidates")
                        .and_then(|value| value.get(0))
                        .and_then(|value| value.get("output"))
                        .unwrap()
                        .to_string();
                    let output = output.replace("\\n", " ");
                    let output = &output[1..output.len()-1];
                    format!("\n{}", output)
                    //output
                },
                reqwest::StatusCode::UNAUTHORIZED => {
                    format!("Error authorizing request")
                }
                _ => {
                    panic!("Unexpected error in API response");
                }
            }
    }
    else {
        "Please provide a valid query.".to_string()
    }
}



pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ai").description("Submit a prompt to Google's LLM")
    .create_option(|option| {
        option
            .name("prompt")
            .description("The prompt you want to pass to the AI model")
            .kind(CommandOptionType::String)
            .required(true)
        });
    return command;
} 