use serenity::builder::CreateApplicationCommand;
use rand::Rng;
use serde_json;

pub fn run() -> String 
{ 
    let mut rng = rand::thread_rng();
    if rng.gen::<f32>() >= 0.50
    {
       //format: noun + verb + er + random numbers 
    }
    else 
    {
       //format noun + verb 
    }
    let json = serde_json::from_str::<serde_json::Value>(&std::fs::read_to_string("./src/words.json")
        .expect("unable to convert file to json"))
        .expect("unable to read file words.json");

    let adjective: String = random_word(json.clone(), String::from("adjective").clone());
    let noun: String = random_word(json.clone(), String::from("noun").clone());
    let verb: String = random_word(json.clone(), String::from("verb").clone());

    
    return String::from("");
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("password").description("Generate a random password between 16-32 characters");
    return command;
} 

pub fn random_word(json: serde_json::Value, word_type: String) -> String
{
    let word: String;
    let mut rng = rand::thread_rng();
    let word_obj = json.get(&word_type);
    match word_obj 
    {
        Some(v) => 
        {
            let word_size = v.as_array().expect("unable to parse words from json").len();
            word = v.get(rng.gen_range(0..word_size)).expect("unable to index through words in json").to_string();
        }
        None => 
        {
            return format!("Unable to parse \"{}\" from json", &word_type);
        }
    }
    return String::from(&word[1..word.len()-1]);
}
