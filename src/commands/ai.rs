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

//trim some markdown that doesn't work with discord specifically
pub fn format_string(output: String) -> String
{
    //trim the quotation marks
    let output = &output[1..output.len()-1];
    //                  this is retarded but it works for discord because it doesn't handle newline
    //                  characters right
    let output = output.replace("\\n", " 
    ");
    //the model tries to escape all quote characters in code, which is wrong, so
    //replace those
    let output = output.replace("\\\"", "\"");
    //replace tab characters not supported by discord
    let output = output.replace("\\t", "");
    let output = output.replace("\\t+", "");
    output
}

//connect to the local LLM model running on desktop
pub async fn run(options: &[CommandDataOption]) -> String {
    dotenv().ok();
    let model : String = env::var("LLM_MODEL").unwrap();
    let server_ip : String = env::var("SERVER_IP").unwrap(); 
    let server_port: String = env::var("SERVER_PORT").unwrap();
    let option = options
        .get(0)
        .expect("Expected prompt option")
        .resolved
        .as_ref()
        .expect("Expected prompt option");
    if let CommandDataOptionValue::String(query) = option {
            let url: String = format!("http://{}:{}/v1/chat/completions", server_ip, server_port);
            let client = reqwest::Client::new();
            let json = &json!(
            {
                "model": model,
                "messages": [ {"role" : "system", "content": query} ],
                "temperature" : 1,
                "max_tokens": 1024,
                "stream" : false,
            });
            let res = client
                .post(url.clone())
                .json(json)
                .send()
                .await
                .expect("Unable to connect to LLM server, Connor's desktop probably isn't running");
            match res.status() {
                reqwest::StatusCode::OK => {
                    println!("Got statuscode OK");
                    let res_text = res.text().await.unwrap();
                    println!("{res_text}");
                    let json_result: Value = serde_json::from_str(&res_text).unwrap();
                    println!("{json_result}");
                    let output = json_result.get("choices")
                        .and_then(|value| value.get(0))
                        .and_then(|value| value.get("message"))
                        .and_then(|value| value.get("content"))
                        .unwrap()
                        .to_string();
                    println!("{:?}", json_result);
                    let output = format_string(output);
                    format!("{}", output)
                },
                reqwest::StatusCode::UNAUTHORIZED => {
                    String::from("Error authorizing request")
                }
                _ => {
                    println!("error: got result {} from api", res.status().to_string());
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
    command
} 
