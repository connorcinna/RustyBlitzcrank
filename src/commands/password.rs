use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::{CommandDataOption, CommandDataOptionValue};
use serenity::model::application::command::CommandOptionType;
use rand::Rng;
use rand::rngs::ThreadRng;
use serde_json;

//TODO: Current the exact same as commands::name, need to add some special password specific stuff
//variable casing, special characters other than 0-9, etc.
pub fn run(options: &[CommandDataOption]) -> String
{
    let mut _size: usize = 0;
    let ret: String;
    let json: serde_json::Value;
    let s: String;
    let option = options
        .get(0)
        .expect("Expected number of characters")
        .resolved
        .as_ref()
        .expect("Expected number of characters");
    if let CommandDataOptionValue::Integer(size) = option { _size = *size as usize; }
    let json_file = std::fs::read_to_string("./src/words.json");
    match json_file
    {
        Ok(json_file) => json = serde_json::from_str::<serde_json::Value>(&json_file)
            .expect("unable to convert file to json"),
        Err(e) => panic!("unable to find json file: {}", e),
    }

    let mut noun: String = random_word(json.clone(), String::from("nouns").clone());
    let rng = rand::thread_rng();
     //format: noun + verb + er + random numbers
    if coinflip(rng.clone())
    {
        let mut verb: String = random_word(json.clone(), String::from("verbs").clone());

        //randomly capitalize some letters otherwise everything is lowercase
        noun = randomize_case(&noun, rng.clone());
        verb = randomize_case(&verb, rng.clone());

        ret = format!("{}{}er", noun, verb);
        s = finalize(ret.clone(), _size, rng.clone());
    }
    //format: adjective + noun + random numbers
    else
    {
        let mut adjective: String = random_word(json.clone(), String::from("adjectives").clone());

        noun = randomize_case(&noun, rng.clone());
        adjective = randomize_case(&adjective, rng.clone());

        ret = format!("{}{}", adjective, noun);
        s = finalize(ret.clone(), _size, rng.clone());
    }
    s
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

pub fn randomize_case(word: &String, rng: ThreadRng)  -> String
{
    let mut ret : String = Default::default();
    for c in word.chars()
    {
        if coinflip(rng.clone())
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

pub fn finalize(mut word: String, _size: usize, mut rng: ThreadRng) -> String
{
    while word.len() < _size as usize
    {
        word.push_str(&rng.gen_range(0..10).to_string());
    }
    let s = match word.char_indices().nth(_size as usize)
    {
        Some((pos, _)) => word[..pos].to_string(),
        None => word.to_string(),
    };
    s
}

pub fn coinflip(mut rng: ThreadRng) -> bool
{
    rng.gen::<f32>() >= 0.50
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("password").description("Generate a random password between 16-32 characters")
        .create_option(|option|
                       {
                            option
                                .name("characters")
                                .description("The number of characters you want your password to be")
                                .kind(CommandOptionType::Integer)
                                .required(true)
                       });
    command
}
