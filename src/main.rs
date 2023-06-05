mod commands;

extern crate dotenv;

use dotenv::dotenv;
use cron::Schedule;
use chrono::{Local, DateTime};

use std::str::FromStr;

use serenity::model::prelude::Activity;
use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::channel::Message;
use serenity::model::id::GuildId;
use serenity::prelude::*;
use serenity::model::prelude::CommandId;

use twitter_v2::TwitterApi;
use twitter_v2::authorization::BearerToken;
use twitter_v2::query::{MediaField, TweetExpansion};


use std::env;


struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        println!("Reached Message Handler");
        let idx_twitter_link = msg.content.find("https://twitter.com");
        if let Some(some_idx) = idx_twitter_link {
            let media_type = vxtwitter(&msg).await;
            if media_type == "Video" {
                let mut final_link = msg.content.to_owned();
                let prepend_str = format!("Posted by {}\n", msg.author.name);
                final_link.insert_str(some_idx+8, "vx"); //guaranteed to not cause buffer issues, already checked string size
                final_link.insert_str(0, &prepend_str);
                msg.channel_id.say(&ctx.http, final_link).await.unwrap();
                msg.delete(&ctx.http).await.expect("Unable to delete message");
            }
        }
    } 

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {

            let content = match command.data.name.as_str() {
                "roll" => commands::roll::run(&command.data.options),
                "gif" => commands::gif::run(&command.data.options).await, //on any commands that need async to run, use await
                "name" => commands::name::run(),
                "search" => commands::search::run(&command.data.options).await,
                "vid" => commands::vid::run(&command.data.options).await,
                "jerma" => commands::jerma::run(),
                "help" => commands::help::run(),
                "gpt" => commands::gpt::run(&command.data.options).await,
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
        ctx.set_activity(Activity::watching("Jerma985")).await;


        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );
        //GuildId::delete_application_command(&guild_id, &ctx.http, CommandId(1104238807907323968)).await.expect("Expected commandID");
        let _guild_commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::roll::register(command))
                .create_application_command(|command| commands::gif::register(command))
                .create_application_command(|command| commands::name::register(command))
                .create_application_command(|command| commands::search::register(command))
                .create_application_command(|command| commands::vid::register(command))
                .create_application_command(|command| commands::jerma::register(command))
                .create_application_command(|command| commands::help::register(command))
                .create_application_command(|command| commands::gpt::register(command))

        })
        .await
        .expect("Could not add the guild command");

//        let global_command = Command::create_global_application_command(&ctx.http, |command| {
//            commands::roll::register(command)
//        })
//        .await;
        
    }
}
fn friday(datetime: DateTime<Local>) {
    println!("It's friday: {}", datetime);
    
}
async fn vxtwitter(rcv_message: &Message) -> String {
    println!("Entering vxtwitter");
    let message_array: Vec<&str> = rcv_message.content.split("/").collect();
    let tweet_id = message_array[message_array.len()-1];
    println!("{tweet_id}");
    let auth = BearerToken::new(env::var("TWITTER_BEARER_TOKEN").expect("Expected Twitter OAuth BearerToken"));
    let tweet = TwitterApi::new(auth)
        .get_tweet(tweet_id.parse::<u64>().unwrap())
        .media_fields([MediaField::Type, MediaField::Url])
        .expansions([TweetExpansion::AttachmentsMediaKeys])
        .send()
        .await
        .unwrap()
        .into_includes()
        .expect("This tweet does not exist");
    println!("Finished grabbing tweet");
    let tweet_media_type = &tweet.media.unwrap()[0].kind;
    format!("{:?}", *tweet_media_type)
}
#[tokio::main]
async fn main() {
    dotenv().ok();
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("CLIENT_TOKEN").expect("Expected a token in the environment");

    // Build our client.
    let mut client = Client::builder(token, GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGES)
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
    let schedule_str = "0 0   10   *   *   Fri *";
    let schedule = Schedule::from_str(schedule_str).unwrap();
    for datetime in schedule.upcoming(Local).take(10) {
        println!("-> {}", datetime);
        friday(datetime);
    }

}