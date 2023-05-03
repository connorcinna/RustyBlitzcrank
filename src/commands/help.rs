use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::{CommandDataOption, CommandDataOptionValue};

pub fn run(options: &[CommandDataOption]) -> String {

}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("help").description("Information about Blitzcrank and it's commands");
    return command;
} 