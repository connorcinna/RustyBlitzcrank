use serenity::builder::CreateApplicationCommand;
use rand::Rng;
use serde_json;


pub fn run() -> String {
    let words = std::fs::read_to_string("./src/words.json").unwrap();
    let words = serde_json::from_str::<serde_json::Value>(&words).unwrap();
    let mut rng = rand::thread_rng();
    let adjective = words.get("adjectives")
        .and_then(|value| value.get(rng.gen_range(0..value.as_array()?.len()))).unwrap().to_string();
    let noun = words.get("nouns")
        .and_then(|value| value.get(rng.gen_range(0..value.as_array()?.len()))).unwrap().to_string();
    let verb = words.get("verbs")
        .and_then(|value| value.get(rng.gen_range(0..value.as_array()?.len()))).unwrap().to_string();
    let adjective = &adjective[1..adjective.len()-1];
    let noun = &noun[1..noun.len()-1];
    let verb = &verb[1..verb.len()-1];

    let num_eos: usize;
    let s_eos: String;
    let coin_flip: f32 = rng.gen();    
    //format: noun + verb + "er" + random numbers
    if coin_flip < 0.50 {
        num_eos = remaining_chars(noun.len() + verb.len());
        s_eos = trailing_num(num_eos);
        let last_char = noun.to_owned().pop().unwrap();
        if last_char == 'e' {
            let total = format!("{noun}{verb}er{s_eos}");
            return total[..16].to_string();
        }
        else {
            let total = format!("{noun}{verb}er{s_eos}");
            //return total[..16].to_string();
            if total.len() > 16 {
                return total[..16].to_string();
            }
            else {
                return total;
            }
        }
    }
    //format: adjective + noun + random numbers
    else {
        num_eos = remaining_chars(adjective.len() + noun.len());
        s_eos = trailing_num(num_eos);
        let total = format!("{adjective}{noun}{s_eos}");
        //return total[..16].to_string();
        if total.len() > 16 {
            return total[..16].to_string();
        }
        else {
            return total;
        }
    }
}

pub fn remaining_chars(input: usize) -> usize {
    if input + 2 >= 16 {
        return 0;
    }
    return 16 - input;
}
pub fn trailing_num(input: usize) -> String {
    let mut s_eos: String = String::from("");
    let mut rng = rand::thread_rng();
    for _i in 0..input {
        let random_num = rng.gen_range(0..10);
        let random_num = random_num.to_string();
        s_eos.push_str(&random_num);
    }
    return s_eos;

}
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("name").description("Generate a random name");
    return command;
} 