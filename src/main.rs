mod commands;

extern crate dotenv;
use dotenv::dotenv;

use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::guild;
use serenity::model::id::GuildId;
use serenity::model::id::CommandId;
use serenity::prelude::*;


use std::env;
use std::fs;

macro_rules! command_match_left {
    ($name: ident) => {
        stringify!($name);
    }
}
macro_rules! command_match_right {

}
macro_rules! command_match {
    ($name:expr) => {
        $name as String => commands::$name::run(&command.data.options)
    };
}
const command_filenames: Vec<String> = fs::read_dir("src/commands").unwrap()
    .filter_map(|result: Result<fs::DirEntry, std::io::Error>| result.ok())
    .map(|filename: fs::DirEntry| -> String {
        filename.path().file_stem().unwrap().to_str().unwrap().to_owned()
    }) 
    .collect::<Vec<_>>();

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {

            let content = match command.data.name.as_str() {
                "roll" => commands::roll::run(&command.data.options),
                _ => "Not implemented".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

//        let command_filenames: Vec<String> = fs::read_dir("src/commands").unwrap()
//            .filter_map(|result: Result<fs::DirEntry, std::io::Error>| result.ok())
//            .map(|filename: fs::DirEntry| -> String {
//                filename.path().file_stem().unwrap().to_str().unwrap().to_owned()
//            }) 
//            .collect::<Vec<_>>();


        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );


        let _guild_commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::roll::register(command))
        })
        .await
        .expect("Could not add the guild command");

//        let global_command = Command::create_global_application_command(&ctx.http, |command| {
//            commands::roll::register(command)
//        })
//        .await;
        
       //TODO: programmatically get name of all commands based on .rs file extension and register them similar to above 
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("CLIENT_TOKEN").expect("Expected a token in the environment");

    // Build our client.
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

}