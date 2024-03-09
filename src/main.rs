mod commands;
pub const MAX_MSG_SZ : usize = 2000;

extern crate dotenv;

use dotenv::dotenv;
use chrono::{Local, DateTime};
use tokio::fs::File;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};
use std::env;

#[allow(deprecated)]
use serenity::model::interactions::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::Activity;
use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::channel::Message;
use serenity::model::id::GuildId;
use serenity::model::id::ChannelId;
use serenity::prelude::*;


struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let idx_twitter_link = msg.content.find("https://twitter.com");
        let idx_x_link = msg.content.find("https://x.com");
        //have to pay for twitter api now so lol
        if let Some(some_idx) = idx_twitter_link {
            vxtwitter(ctx, &msg, (some_idx, "twitter")).await;
        }
        else if let Some(some_idx) = idx_x_link {
            vxtwitter(ctx, &msg, (some_idx, "x")).await;
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = &interaction {
            let cmd_str = command.data.name.as_str();
            match cmd_str {
               "search" => special_interaction(ctx, &interaction).await,
               "ai" => special_interaction(ctx, &interaction).await,
               "password" => special_interaction(ctx, &interaction).await,
                _ => normal_interaction(ctx, &interaction).await
            };
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
        let test_guild_id = GuildId(
            env::var("TEST_GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        //add commands to the main server
        let _guild_commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::roll::register(command))
                .create_application_command(|command| commands::gif::register(command))
                .create_application_command(|command| commands::name::register(command))
                .create_application_command(|command| commands::search::register(command))
                .create_application_command(|command| commands::vid::register(command))
                .create_application_command(|command| commands::jerma::register(command))
                .create_application_command(|command| commands::help::register(command))
                .create_application_command(|command| commands::song::register(command))
                .create_application_command(|command| commands::ping_voice::register(command))
                .create_application_command(|command| commands::ai::register(command))
                .create_application_command(|command| commands::password::register(command))
        })
        .await
        .expect("Could not add the guild command");

        //add commands to the test server
        let _test_guild_commands = GuildId::set_application_commands(&test_guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::roll::register(command))
                .create_application_command(|command| commands::gif::register(command))
                .create_application_command(|command| commands::name::register(command))
                .create_application_command(|command| commands::search::register(command))
                .create_application_command(|command| commands::vid::register(command))
                .create_application_command(|command| commands::jerma::register(command))
                .create_application_command(|command| commands::help::register(command))
                .create_application_command(|command| commands::song::register(command))
                .create_application_command(|command| commands::ping_voice::register(command))
                .create_application_command(|command| commands::ai::register(command))
                .create_application_command(|command| commands::password::register(command))
        })
        .await
        .expect("Could not add the guild command");

        let schedule = JobScheduler::new().await.unwrap();
        schedule.add(
            Job::new("0 0 14 * *  Fri *", move |_uuid, _l| { // 2PM UTC => 9AM EST
                let channel_id = ChannelId(
                    env::var("MAIN_CHANNEL_ID")
                    .expect("Expected MAIN_CHANNEL_ID in environment")
                    .parse()
                    .expect("MAIN_CHANNEL_ID must be an integer"));
                channel_id.send_message(&ctx.http, |message| message.content("https://www.youtube.com/watch?v=WUyJ6N6FD9Q"))
                    .await;
            }).unwrap(),
        ).await.unwrap();
    }
}

async fn vxtwitter(ctx: Context, msg: &Message, some_idx: (usize, &str)) {
    let mut final_link = msg.content.to_owned();
    let prepend_str = format!("Posted by {}\n", msg.author.name);
    //TODO: maybe do some fancy url object magic to change the url string instead of just using it
    //like a string
    if some_idx.1 == "twitter" { //twitter link
        final_link.insert_str(some_idx.0+8, "c.vx"); //guaranteed to not panic, already checked string size
    }
    else { //x link
        final_link.insert_str(some_idx.0+8, "c.v");
        final_link.insert_str(some_idx.0+12, "twitter");
    }
    final_link.insert_str(0, &prepend_str);

    msg.channel_id.say(&ctx.http, final_link).await.unwrap();
    msg.delete(&ctx.http).await.expect("Unable to delete message");
}

async fn normal_interaction(ctx: Context, interaction: &Interaction) {
    if let Interaction::ApplicationCommand(command) = &interaction {
        let cmd_str = command.data.name.as_str();
        let content = match cmd_str {
            "roll" => commands::roll::run(&command.data.options),
            "gif" => commands::gif::run(&command.data.options).await,
            "name" => commands::name::run(),
            "vid" => commands::vid::run(&command.data.options).await,
            "jerma" => commands::jerma::run(),
            "help" => commands::help::run(),
            "song" => commands::song::run(&command.data.options).await,
            "ping_voice" => commands::ping_voice::run(ctx.clone(), &command.data.options).await,
            _ => "Not implemented".to_string(),
        };
        if let Err(e) = command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content(content))
            })
            .await
        {
            println!("Cannot respond to slash command: {}", e);
        }
    }

}
async fn special_interaction(ctx: Context, interaction: &Interaction) {
    if let Interaction::ApplicationCommand(command) = &interaction {
        let cmd_str = command.data.name.as_str();
        match cmd_str {
           "search" => search_interaction(ctx, command).await,
           "ai" => ai_interaction(ctx, command).await,
           "password" => password_interaction(ctx, command).await,
           &_ => println!("Unimplemented"),
        }
    }
}
#[allow(deprecated)]
async fn search_interaction(ctx: Context, command: &ApplicationCommandInteraction) {
    command.create_interaction_response(&ctx.http, |response| {
    response
        .kind(InteractionResponseType::DeferredChannelMessageWithSource)
        .interaction_response_data(|message| message.content(command.data.name.as_str()))
    }).await.unwrap();
    let res = commands::search::run(&command.data.options).await;
    if res == "Fuck" { //google didn't find anything
        let channel_id = command.channel_id;
        let mut img_path = std::env::current_dir().unwrap();
        img_path.push("lol.png");
        let img_file = File::open(img_path).await.unwrap();
        let files = vec![(&img_file, "lol.png")];
        //funny blitzcrank picture
        //empty message closure to satisfy function
        let _ = channel_id.send_files(&ctx.http, files, |m| m).await;
        //get rid of the "bot is thinking..." message
        command.delete_original_interaction_response(&ctx.http).await.unwrap();
    }
    else { //normal case
        command.edit_original_interaction_response(&ctx.http, |response| {
            response.content(res)
        }).await.unwrap();
    }   
}

#[allow(deprecated)]
async fn ai_interaction(ctx: Context, command: &ApplicationCommandInteraction) {
    command.create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::DeferredChannelMessageWithSource)
                .interaction_response_data(|message| message.content(command.data.name.as_str()))
            }).await.unwrap();
    let res = commands::ai::run(&command.data.options).await;
    if res.chars().count() >= MAX_MSG_SZ {
        let char_vec: Vec<char> = res.chars().collect();
        let first_message_str: String = char_vec[..MAX_MSG_SZ].into_iter().collect();
        let second_message_str: String = char_vec[MAX_MSG_SZ..].into_iter().collect();
        command.edit_original_interaction_response(&ctx.http, |response| {
            response.content(&first_message_str)
        }).await.unwrap();
        command.create_followup_message(&ctx.http, |response| {
            response.content(&second_message_str)
        }).await.unwrap();
    }
    else {
        command.edit_original_interaction_response(&ctx.http, |response| {
            response.content(&res)
        }).await.unwrap();
    }
}
#[allow(deprecated)]
async fn password_interaction(ctx: Context, command: &ApplicationCommandInteraction) {
    let res = commands::password::run(&command.data.options);
    let dm = command.user.direct_message(&ctx, |message| {
        message.content(format!("||{}||", res))
    }).await;
    match dm {
        Ok(_) => println!("Successfully sent dm to {} with new password", command.user.name),
        Err(e) => println!("Error sending DM to {} : {}", command.user.name, e)
    }
    command.create_interaction_response(&ctx.http, |response| {
        response.interaction_response_data(|message| message.content("Sent, check your direct messages"))
    }).await.unwrap();
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Configure the client with the Discord bot token in the environment.
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
//    let schedule = Schedule::from_str( "0 0 10 * *  Fri *").unwrap();
//    for datetime in schedule.upcoming(Local).take(10) {
//        println!("-> {}", datetime);
//        friday(datetime);
//    }

}
