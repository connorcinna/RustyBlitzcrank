use serenity::builder::CreateApplicationCommand;

use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::{CommandDataOption, CommandDataOptionValue};
use serenity::model::channel::ChannelType;
use serenity::model::id::ChannelId;
use serenity::model::mention::Mention;

use serenity::client::Context;

extern crate dotenv;
extern crate serde_json;


pub async fn run(ctx: Context, options: &[CommandDataOption], source_channel: ChannelId) -> String {
    let channel = options
            .get(0)
            .expect("Expected channel")
            .resolved
            .as_ref()
            .expect("Expected channel");
    let mut message: Option<&CommandDataOptionValue> = None;
    if options.len() > 1 {
        message = options
            .get(1)
            .expect("Expected message")
            .resolved
            .as_ref()
    }
    

    if let CommandDataOptionValue::Channel(channel) = channel {
        let id = channel.id;
        let to_channel = id.to_channel(&ctx.http).await.unwrap();
        let guild = to_channel.guild().unwrap();
        let mut s: String = String::from("");
        if let Ok(members) = guild.members(ctx.cache).await {
            //mention all of the users in the channel
            for member in members {
                s.push_str(Mention::from(member.user.id).to_string().as_str());
            }
            //check if the user attached a message with the command
            match message {
                Some(CommandDataOptionValue::String(message)) => {
                    s.push_str(message.as_str());
                },
                None => {},
                _ => {}
            }
            return s;
        }
        else {
            return String::from("No one detected in the voice channel");
        }
    }
    String::from("Unexpected error")
}


pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping_voice")
    .description("@ everyone currently talking in a voice channel")
    .create_option(|option| {
        option
            .name("channel")
            .description("The voice channel to address")
            .kind(CommandOptionType::Channel)
            .channel_types(&[ChannelType::Voice])
            .required(true)
    })
    .create_option(|option| {
        option
            .name("message")
            .description("Message you'd like to include in the channel mention")
            .kind(CommandOptionType::String)
    });
    return command;
}
