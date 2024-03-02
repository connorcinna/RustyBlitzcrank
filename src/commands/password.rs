use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::{CommandDataOption, CommandDataOptionValue};
use serenity::model::application::command::CommandOptionType;
use rand::Rng;
use serde_json;

pub fn run(options: &[CommandDataOption]) -> String 
{ 
    let mut _size: i64 = 0;
    let mut ret: String;
    let option = options
        .get(0)
        .expect("Expected number of characters")
        .resolved
        .as_ref()
        .expect("Expected number of characters");
    if let CommandDataOptionValue::Integer(size) = option { _size = *size; }
    let json = serde_json::from_str::<serde_json::Value>(&std::fs::read_to_string("./src/words.json")
        .expect("unable to convert file to json"))
        .expect("unable to read file words.json");
    let noun: String = random_word(json.clone(), String::from("noun").clone());
    let mut rng = rand::thread_rng();
    if rng.gen::<f32>() >= 0.50
    {
         //format: noun + verb + er + random numbers 
        let verb: String = random_word(json.clone(), String::from("verb").clone());
        ret = format!("{noun}{verb}er");
        while ret.len() < _size as usize
        {
            ret.push_str(&rng.gen_range(0..10).to_string());
        }
        return ret[.._size as i32].to_string();
    }
    else 
    {
        let adjective: String = random_word(json.clone(), String::from("adjective").clone());
       //format: adjective + noun + random numbers
    }
    
    String::from("")
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
    String::from(&word[1..word.len()-1])
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("password").description("Generate a random password between 16-32 characters")
        .create_option(|option| 
                       {
                            option
                                .name("Characters")
                                .description("The number of characters you want your password to be")
                                .kind(CommandOptionType::Integer)
                                .required(true)
                       });
    command
}
