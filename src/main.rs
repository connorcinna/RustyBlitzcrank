mod commands;
mod common;
mod websites;
extern crate dotenv;

use std::env;
use dotenv::dotenv;
use tokio::fs::File;
use tokio_cron_scheduler::{Job, JobScheduler};
use poise::serenity_prelude::EventHandler;
use poise::serenity_prelude as serenity;

use crate::websites::{Website, LinkFix};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

struct Handler;
struct Data {}

async fn fix_links(old_link: String, new_link: String, msg: &Message, ctx: &Context)
{
    let mut final_link = msg.content.to_owned();
    let prepend_str = format!("Posted by {}\n", msg.author.name);
    final_link = final_link.replace(&old_link, &new_link);
    final_link.insert_str(0, &prepend_str);
    msg.channel_id.say(&ctx.http, final_link).await.unwrap();
    msg.delete(&ctx.http).await.expect("Unable to delete message");
}

impl serenity::EventHandler for Handler
{
    async fn message(&self, ctx: Context, msg: Message)
    {
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

    async fn interaction_create(&self, ctx: Context, interaction: serenity::Interaction)
    {
        if let serenity::Interaction::Command(command) = &interaction
        {
            let cmd_str = command.data.name.as_str();
            match cmd_str
            {
               "search" => special_interaction(ctx, &interaction).await,
               "ai" => special_interaction(ctx, &interaction).await,
               "password" => special_interaction(ctx, &interaction).await,
                _ => normal_interaction(ctx, &interaction).await
            };
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready)
    {
        println!("{} is connected!", ready.user.name);
        ctx.set_activity(Some(ActivityData::custom("1459 days of trump left")));

        //TODO: poise::builtins::register_globally a vec! of commands

        match JobScheduler::new().await
        {
            Ok(schedule) =>
            {
                let channel_id = ChannelId(
                    env::var("MAIN_CHANNEL_ID")
                    .expect("Expected MAIN_CHANNEL_ID in environment")
                    .parse()
                    .expect("MAIN_CHANNEL_ID must be an integer"));
                //async closures don't really work, have to make the inner closure create a future and then
                //let tokio handle executing it
                schedule.add(
                    Job::new("0 0 14 * *  Fri *", move |_uuid, _l| // 2PM UTC => 9AM EST
                    {
                        let rt = tokio::runtime::Runtime::new().unwrap();
                        let future = channel_id.send_message(ctx.http.clone(), |message| message.content("https://www.youtube.com/watch?v=WUyJ6N6FD9Q"));
                        let _ = rt.block_on(future);
                    }).unwrap(),
                ).await.unwrap();
            },
            Err(e) => panic!("Unable to initialize JobScheduler: {}", e),
        };
    }
}

async fn normal_interaction(ctx: Context, interaction: &Interaction)
{
    if let Interaction::ApplicationCommand(command) = &interaction
    {
        let cmd_str = command.data.name.as_str();
        let content = match cmd_str {
            "roll" => commands::roll::run(&command.data.options),
            "gif" => commands::gif::run(&command.data.options).await,
            "name" => commands::name::run(&command.data.options),
            "vid" => commands::vid::run(&command.data.options).await,
            "jerma" => commands::jerma::run(),
            "help" => commands::help::run(),
            "song" => commands::song::run(&command.data.options).await,
            "ping_voice" => commands::ping_voice::run(ctx.clone(), &command.data.options).await,
            "freaky" => commands::freaky::run(&command.data.options),
            _ => "Not implemented".to_string(),
        };
        if let Err(e) = command.create_interaction_response(&ctx.http, |response|
        {
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
async fn special_interaction(ctx: Context<'_>, interaction: &Interaction)
{
    if let Interaction::ApplicationCommand(command) = &interaction
    {
        let cmd_str = command.data.name.as_str();
        match cmd_str
        {
           "search" =>
           {
               if !commands::search::interaction(&ctx, command).await
               {
                   no_results(ctx, command).await;
               }
           },
           "ai" => commands::ai::interaction(ctx, command).await,
           "password" => commands::password::interaction(ctx, command).await,
           &_ => println!("Unimplemented"),
        }
    }
}

#[allow(deprecated)]
async fn no_results(ctx: Context<'_>, command: &ApplicationCommandInteraction)
{
    let channel_id = command.channel_id;
    let mut img_path = std::env::current_dir().unwrap();
    img_path.push("resources/lol.png");
    let img_file = File::open(img_path).await.unwrap();
    let files = vec![(&img_file, "resources/lol.png")];
    //empty message closure to satisfy function
    let _ = channel_id.send_files(&ctx.http, files, |m| m).await;
    //get rid of the "bot is thinking..." message
    command.delete_original_interaction_response(&ctx.http).await.unwrap();
}

#[tokio::main]
async fn main()
{
    dotenv().ok();
    // Configure the client with the Discord bot token in the environment.
    let token = env::var("CLIENT_TOKEN").expect("Expected a token in the environment");
    let intents = serenity::GatewayIntents::privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions
        {
            commands: vec!
            [
                commands::ai::send_prompt(),
                commands::freaky::run(),
                commands::gif::run(),
                commands::help::run(),
                commands::jerma::run(),
                commands::name::run(),
                commands::password::run(),
                commands::ping_voice::run(),
                commands::roll::run(),
                commands::search::run(),
                commands::song::run(),
                commands::vid::run(),
            ],
            on_error: |error|
            {
                Box::pin(async move
                {
                    match error
                    {
                        poise::FrameworkError::ArgumentParse { error, .. } =>
                        {
                            println!("Error parsing arguments to Poise framework builder");
                        }
                        other => poise::builtins::on_error(other).await.unwrap(),
                    }

                })
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework|
        {
            Box::pin(async move
            {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                    Ok(Data{})
            })
        })
        .build();
    // Build our client.
    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
