use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{CommandDataOption, CommandDataOptionValue};
use rand::Rng;

pub fn run(options: &[CommandDataOption]) -> String {
    let option = options
        .get(0)
        .expect("Expected ceiling option")
        .resolved
        .as_ref()
        .expect("Expected ceiling option");
    if let CommandDataOptionValue::Integer(ceiling) = option {
        let mut rng = rand::thread_rng();
        let roll_result: i64 = rng.gen_range(0..*ceiling);
        format!("{}", roll_result) 
    }
    else {
        "Please provide a valid ceiling value.".to_string()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("roll").description("Roll a dice for a random value between 1 and the value given in the argument, e.g. /roll 50")
    .create_option(|option| {
        option
            .name("ceiling")
            .description("The maximum value of the dice roll.")
            .kind(CommandOptionType::Integer)
            .required(true)
        });
    return command;
} 