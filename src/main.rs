mod commands;

extern crate dotenv;
use dotenv::dotenv;

use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
//use serenity::model::guild;
use serenity::model::id::GuildId;
use serenity::model::id::CommandId;
use serenity::prelude::*;


use std::env;


struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {

            let content = match command.data.name.as_str() {
                "roll" => commands::roll::run(&command.data.options),
                "gif" => commands::gif::run(&command.data.options).await, //on any commands that need async to run, use await
//                "name" => commands::name::run(&command.data.options),
                "search" => commands::search::run(&command.data.options).await,
                "vid" => commands::vid::run(&command.data.options).await,
                "jerma" => commands::jerma::run(),
//                "help" => commands::help::run(&command.data.options),
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


        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );
        GuildId::delete_application_command(&guild_id, &ctx.http, CommandId(1100989549800333392)).await.expect("Expected commandID");
        let _guild_commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::roll::register(command))
                .create_application_command(|command| commands::gif::register(command))
//                .create_application_command(|command| commands::name::register(command))
                .create_application_command(|command| commands::search::register(command))
                .create_application_command(|command| commands::vid::register(command))
                .create_application_command(|command| commands::jerma::register(command))
//                .create_application_command(|command| commands::help::register(command))
        })
        .await
        .expect("Could not add the guild command");

//        let global_command = Command::create_global_application_command(&ctx.http, |command| {
//            commands::roll::register(command)
//        })
//        .await;
        
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