extern crate dotenv;
extern crate serde_json;
use dotenv::dotenv;
use serde_json::Value;
use serde_json::json;
use std::time::Duration;
use std::env;
use reqwest;
use crate::{Context, Error};


static TIMEOUT : u64 = 30;
static SYSTEM_PROMPT : &str = "You are the robot Blitzcrank from League of Legends. Answer all questions in all cap letters";

//trim some markdown that doesn't work with discord specifically
pub fn format_string(output: String) -> String
{
    //trim the quotation marks
    let output = &output[1..output.len()-1];
    //this is retarded but it works for discord because it doesn't handle newline
    //characters right
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

//connect to the model by http
#[poise::command(slash_command)]
pub async fn send_prompt(
    ctx: Context<'_>,
    #[description = "The prompt to pass to the LLM"] prompt: String
    ) -> Result<(), Error>
{
    dotenv().ok();
    let model : String = env::var("LLM_MODEL").unwrap();
    let server_ip : String = env::var("SERVER_IP").unwrap();
    let server_port: String = env::var("SERVER_PORT").unwrap();
    let url: String = format!("http://{}:{}/v1/chat/completions", server_ip, server_port);
    let json = &json!(
    {
        "model": model,
        "messages":
            [
                {"role" : "system", "content": SYSTEM_PROMPT},
                {"role" : "user", "content": prompt},
            ],
        "temperature" : 1,
        "max_tokens": 1024,
        "stream" : false,
    });
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(TIMEOUT))
        .build()
        .unwrap();
    let mut success : bool = false;
    let res = client
        .post(url.clone())
        .json(json)
        .send()
        .await
        .and_then(|v| { success = true; Ok(v) });
    if !success
    {
        let _ = ctx.say("Unable to connect to LLM server, Connor's desktop probably isn't running").await;
        return Ok(())
    }
    let res = res.unwrap();
    match res.status()
    {
        reqwest::StatusCode::OK =>
        {
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
            let _ = ctx.say(format_string(output)).await;
            Ok(())
        },
        reqwest::StatusCode::UNAUTHORIZED =>
        {
            let _ = ctx.say("Error authorizing request").await;
            Ok(())
        }
        _ =>
        {
            let _ = ctx.say(format!("error: got result {} from api", res.status().to_string())).await;
            Ok(())
        }
    }
}

//pub async fn interaction(ctx: Context<'_>, command: &ApplicationCommandInteraction)
//{
//    command.create_interaction_response(&ctx.http, |response|
//    {
//        response
//            .kind(InteractionResponseType::DeferredChannelMessageWithSource)
//            .interaction_response_data(|message| message.content(command.data.name.as_str()))
//    }).await.unwrap();
//    let res = run(&command.data.options).await;
//    if res.chars().count() >= MAX_MSG_SZ
//    {
//        let char_vec: Vec<char> = res.chars().collect();
//        let first_message_str: String = char_vec[..MAX_MSG_SZ].into_iter().collect();
//        let second_message_str: String = char_vec[MAX_MSG_SZ..].into_iter().collect();
//        command.edit_original_interaction_response(&ctx.http, |response| {
//            response.content(&first_message_str)
//        }).await.unwrap();
//        command.create_followup_message(&ctx.http, |response| {
//            response.content(&second_message_str)
//        }).await.unwrap();
//    }
//    else
//    {
//        command.edit_original_interaction_response(&ctx.http, |response|
//        {
//            response.content(&res)
//        }).await.unwrap();
//    }
//}
