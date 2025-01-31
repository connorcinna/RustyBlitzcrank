use crate::common::helpers::coinflip;

use rand::Rng;
use serde_json;
pub static _SIZE : usize = 16;
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn run(
    ctx: Context<'_>,
    #[description = "The number of names you wish to generate"] num: Option<u32>) -> Result<(), Error>
{
    let mut name: String = String::new();
    match num
    {
        //if a number was passed in
        Some(num) =>
        {
            for _ in 0..num
            {
                name += &generate_name();
                name += "\n";
            }
        }
        None =>
        {
            name = generate_name();
        }
    }
    ctx.say(name);
    Ok(())
}

 //format: noun + verb + er + random numbers
pub fn generate_format_one(json: serde_json::value::Value, noun: String) -> String
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
pub fn generate_format_two(json: serde_json::value::Value, noun: String) -> String
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

pub fn generate_name() -> String
{
    let s: String;
    let json = open_json("resources/words.json");
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

pub fn open_json(path: &str) -> serde_json::value::Value
{
    let json: serde_json::Value;
    let json_string = std::fs::read_to_string(path);
    match json_string
    {
        Ok(json_string) => json = serde_json::from_str::<serde_json::Value>(&json_string)
            .expect("unable to convert file to json"),
        Err(e) => panic!("unable to find json file: {}", e),
    }
    return json;
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
