use serenity::builder::CreateApplicationCommand;
pub fn run() -> String {
    String::from("https://github.com/connorcinna/RustyBlitzcrank#readme")
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("help").description("Information about Blitzcrank and it's commands");
    command
} 