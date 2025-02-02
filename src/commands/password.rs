use rand::prelude::*;
use serde_json;

use crate::{Context, Error};
use poise::serenity_prelude::CreateMessage;
use crate::common::helpers::coinflip;

//TODO: refactor this and name.rs to some common file
//TODO: really need to read the json into string and share it by reference without cloning, would
//be way more efficient
#[poise::command(slash_command, rename = "password")]
pub async fn run(
    ctx: Context<'_>,
    #[description = "Number of characters in the password"] size: i64
) -> Result<(), Error>
{
    let _ = ctx.defer().await;
    let json: serde_json::Value;
    let s: String;
    let json_file = std::fs::read_to_string("./resources/words.json");
    match json_file
    {
        Ok(json_file) => json = serde_json::from_str::<serde_json::Value>(&json_file)
            .expect("unable to convert file to json"),
        Err(e) => panic!("unable to find json file: {}", e),
    }
    let noun: String = randomize_case(&random_word(json.clone(), String::from("nouns").clone()));
    if coinflip()
    {
        s = generate_format_one(json, noun, size);
    }
    else
    {
        s = generate_format_two(json, noun, size);
    }
    let s = finalize(s, size);
    let builder = CreateMessage::new().content(format!("||{}||", s));
    let dm = ctx.author().direct_message(&ctx, builder).await;
    match dm
    {
        Ok(_) =>
        {
            println!("Successfully sent dm to {} with new password", ctx.author().name);
            let _ = ctx.say("Sent, check your direct messages").await;
            return Ok(());
        }
        Err(e) =>
        {
            println!("Error sending dm to {} : {}", ctx.author().name, e);
            let _ = ctx.say("Unable to send DM").await;
            return Err("Unable to send DM".into());
        }
    }
}

pub fn random_word(json: serde_json::Value, word_type: String) -> String
{
    let word: String;
    let mut rng = rand::rng();
    let word_obj = json.get(&word_type);
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

pub fn randomize_case(word: &String)  -> String
{
    let mut ret : String = Default::default();
    for c in word.chars()
    {
        if coinflip()
        {
            ret.push(c.to_ascii_uppercase());
        }
        else
        {
            ret.push(c);
        }
    }
    ret
}

pub fn finalize(mut word: String, _size: i64) -> String
{
    let special_chars = ["!", "@", "#", "$", "%", "^", "&", "*", "(", ")", "?", "[", "]"];
    while word.len() < _size as usize
    {
        let index: usize =
        {
            let mut rng = rand::rng();
            rng.random_range(0..special_chars.len())
        };
        if coinflip()
        {
            word.push_str(&index.to_string());
        }
        else
        {
            word.push_str(special_chars[index]);
        }
    }
    let s = match word.char_indices().nth(_size as usize)
    {
        Some((pos, _)) => word[..pos].to_string(),
        None => word.to_string(),
    };
    s
}

pub fn generate_format_one(json: serde_json::value::Value, noun: String, size: i64) -> String
{
    let verb: String = randomize_case(&random_word(json.clone(), String::from("verbs").clone()));
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
    while ret.len() < size as usize
    {
        ret.push_str(&rng.random_range(0..10).to_string());
    }
    return String::from(ret);

}

pub fn generate_format_two(json: serde_json::value::Value, noun: String, size: i64) -> String
{
    let adjective: String = random_word(json.clone(), String::from("adjectives").clone());
    let mut rng = rand::rng();
    let mut ret = format!("{}{}", adjective, noun);
    while ret.len() < size as usize
    {
        ret.push_str(&rng.random_range(0..10).to_string());
    }
    return String::from(ret);
}
