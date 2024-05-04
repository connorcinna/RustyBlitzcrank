mod commands;
mod websites;
mod constants;

use crate::websites::{Website, LinkFix};
use crate::constants::MAX_MSG_SZ;

extern crate dotenv;

use dotenv::dotenv;
use tokio::fs::File;
use tokio_cron_scheduler::{Job, JobScheduler};
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

async fn fix_links(old_link: String, new_link: String, msg: &Message, ctx: &Context)
{
    let mut final_link = msg.content.to_owned();
    let prepend_str = format!("Posted by {}\n", msg.author.name);
    final_link = final_link.replace(&old_link, &new_link);
    final_link.insert_str(0, &prepend_str);
    msg.channel_id.say(&ctx.http, final_link).await.unwrap();
    msg.delete(&ctx.http).await.expect("Unable to delete message");
}

#[async_trait]
impl EventHandler for Handler {

    async fn message(&self, ctx: Context, msg: Message) {
        let links : [LinkFix; 5] = 
        [
            LinkFix {website: Website::Twitter, old_link: String::from("https://twitter.com"), new_link: String::from("https://vxtwitter.com")},
            LinkFix {website: Website::X, old_link: String::from("https://x.com"), new_link: String::from("https://c.vxtwitter.com")},
            LinkFix {website: Website::Tiktok, old_link: String::from("https://www.tiktok.com"), new_link: String::from("https://vxtiktok.com")},
            LinkFix {website: Website::Instagram, old_link: String::from("https://www.instagram.com"), new_link: String::from("https://ddinstagram.com")},
            LinkFix {website: Website::Reddit, old_link: String::from("https://www.reddit.com"), new_link: String::from("https://vxreddit.com")},
        ];
        for link in links
        {
            if msg.content.find(&link.old_link).is_some()
            {
                println!("fixing link: {0}", link.old_link.clone());
                fix_links(link.old_link.clone(), link.new_link.clone(), &msg, &ctx).await;
            }
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
        match JobScheduler::new().await {
            Ok(schedule) => {
                let channel_id = ChannelId(
                    env::var("MAIN_CHANNEL_ID")
                    .expect("Expected MAIN_CHANNEL_ID in environment")
                    .parse()
                    .expect("MAIN_CHANNEL_ID must be an integer"));
                //async closures don't really work, have to make the inner closure create a future and then
                //let tokio handle executing it
                schedule.add(
                    Job::new("0 0 14 * *  Fri *", move |_uuid, _l| { // 2PM UTC => 9AM EST
                        let rt = tokio::runtime::Runtime::new().unwrap();
                        let future = channel_id.send_message(ctx.http.clone(), |message| message.content("https://www.youtube.com/watch?v=WUyJ6N6FD9Q"));
                        let _ = rt.block_on(future);
                    }).unwrap(),
                ).await.unwrap();
            },
            Err(e) => panic!("Unable to initialize JobScheduler: {}", e),
        } ;
    }
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

//handle interactions that require doing some extra stuff other than just sending to the channel
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

//handle deferring the message, wait for the response from API call, and send it to the channel
//or, if the API didn't find an image link, send the funny blitzcrank picture
#[allow(deprecated)]
async fn search_interaction(ctx: Context, command: &ApplicationCommandInteraction) {
    command.create_interaction_response(&ctx.http, |response| {
    response
        .kind(InteractionResponseType::DeferredChannelMessageWithSource)
        .interaction_response_data(|message| message.content(command.data.name.as_str()))
    }).await.unwrap();
    if let Ok(res) = commands::search::run(&command.data.options).await {
        command.edit_original_interaction_response(&ctx.http, |response| response.content(res)).await.unwrap();
    }
    else {
        no_results(ctx, command).await;
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
#[allow(deprecated)]
async fn no_results(ctx: Context, command: &ApplicationCommandInteraction) {
    let channel_id = command.channel_id;
    let mut img_path = std::env::current_dir().unwrap();
    img_path.push("lol.png");
    let img_file = File::open(img_path).await.unwrap();
    let files = vec![(&img_file, "lol.png")];
    //empty message closure to satisfy function
    let _ = channel_id.send_files(&ctx.http, files, |m| m).await;
    //get rid of the "bot is thinking..." message
    command.delete_original_interaction_response(&ctx.http).await.unwrap();
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
}
