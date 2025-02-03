use crate::common::helpers::coinflip;

use rand::Rng;
use serde_json;
pub static _SIZE : usize = 16;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use poise::reply::CreateReply;
use std::path::Path;
use poise::serenity_prelude as serenity;
use crate::{Context, Error};


#[poise::command(slash_command, rename = "name")]
pub async fn run(
    ctx: Context<'_>,
    #[description = "The number of names you wish to generate"] num: Option<i64>) -> Result<(), Error>
{
    let _ = ctx.defer().await;
    let mut name: String = String::new();
    let json = open_json("resources/words.json").unwrap();
    match num
    {
        Some(num) =>
        {
            for _ in 0..num
            {
                name += &generate_name(&json);
                name += "\n";
            }
        }
        None =>
        {
            name = generate_name(&json);
        }
    }
    if name.chars().count() >= 2000
    {
        //create .txt file embed with the string
        let _ = create_file_embed(ctx, &name).await?;
    }
    else
    {
        let _ = ctx.say(name).await;
    }
    Ok(())
}

 //format: noun + verb + er + random numbers
pub fn generate_format_one(json: &serde_json::value::Value, noun: String) -> String
{
    let verb: String = random_word(json.clone(), String::from("verbs").clone());
    let mut rng = rand::rng();
    let mut ret : String;
    let last_chars =
    {
        let split_pos = verb.char_indices().nth_back(2).unwrap().0;
        &verb[..split_pos]
    };
    if last_chars == "er"
    {
        ret = format!("{}{}", noun, verb);
    }
    else if verb.chars().last().unwrap() == 'e'
    {
        ret = format!("{}{}r", noun, verb);
    }
    else
    {
        ret = format!("{}{}er", noun, verb);
    }
    //append random numbers to the end
    while ret.len() < _SIZE as usize
    {
        ret.push_str(&rng.random_range(0..10).to_string());
    }
    return String::from(ret);
}

//format: adjective + noun + random numbers
pub fn generate_format_two(json: &serde_json::value::Value, noun: String) -> String
{
    let adjective: String = random_word(json.clone(), String::from("adjectives").clone());
    let mut rng = rand::rng();
    let mut ret = format!("{}{}", adjective, noun);
    while ret.len() < _SIZE as usize
    {
        ret.push_str(&rng.random_range(0..10).to_string());
    }
    return String::from(ret);
}

pub fn generate_name(json: &serde_json::value::Value) -> String
{
    let s: String;
    let noun: String = random_word(json.clone(), String::from("nouns").clone());
    if coinflip()
    {
        s = generate_format_one(json, noun);
    }
    else
    {
        s = generate_format_two(json, noun);
    }
    s
}

pub fn open_json(path: &str) -> Result<serde_json::value::Value, Error>
{
    let json: serde_json::Value;
    let json_string = std::fs::read_to_string(path);
    match json_string
    {
        Ok(json_string) => json = serde_json::from_str::<serde_json::Value>(&json_string)
            .expect("unable to convert file to json"),
        Err(e) => return Err(Box::new(e)),
    }
    return Ok(json);
}

pub fn random_word(json: serde_json::Value, word_type: String) -> String
{
    let word: String;
    let word_obj = json.get(&word_type);
    let mut rng = rand::rng();
    match word_obj
    {
        Some(v) =>
        {
            let word_size = v.as_array().expect("unable to parse words from json").len();
            word = v.get(rng.random_range(0..word_size)).expect("unable to index through words in json").to_string();
        }
        None =>
        {
            return format!("Unable to parse \"{}\" from json", &word_type);
        }
    }
    String::from(&word[1..word.len()-1])
}

async fn create_file_embed(ctx: Context<'_>, name: &str) -> Result<(), Error>
{
    let file_name = "/tmp/names.txt";
    let mut file = File::create(file_name).await?;
    file.write_all(name.as_bytes()).await?;
    let attachment = serenity::CreateAttachment::path(Path::new(file_name)).await?;
    let _ = ctx.send(CreateReply::default()
        .attachment(attachment))
        .await?;
    Ok(())

}
