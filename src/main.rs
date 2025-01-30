mod commands;
mod common;
mod websites;
extern crate dotenv;

use std::{env, future, pin};
use dotenv::dotenv;
use tokio::fs::File;
use tokio_cron_scheduler::{Job, JobScheduler};
use chrono::{DateTime, Utc, TimeDelta};
use poise::serenity_prelude::EventHandler;
use poise::serenity_prelude as serenity;
use poise::async_trait;
use serenity::Interaction;

use crate::websites::{Website, LinkFix};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

struct Handler;
struct Data {}
//TODO move all this extra date and struct stuff somewhere else
const YEAR_IN_SECONDS: f64 = 60.0 * 60.0 * 24.0 * (365.0 + 0.25);
const MONTH_IN_SECONDS: f64 = YEAR_IN_SECONDS / 12.0;
const WEEK_IN_SECONDS: f64 = MONTH_IN_SECONDS / 4.345;
const DAY_IN_SECONDS: f64 = WEEK_IN_SECONDS / 7.0;
const HOUR_IN_SECONDS: f64 = DAY_IN_SECONDS / 24.0;
const MIN_IN_SECONDS: f64 = HOUR_IN_SECONDS / 60.0;

//simple struct to hold break down of TimeDelta
#[derive(Debug)]
struct TimeData
{
    years: i64,
    months: i64,
    weeks: i64,
    days: i64,
    hours: i64,
    minutes: i64,
    seconds: i64,
}


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
impl EventHandler for Handler
{
    async fn message(&self, ctx: serenity::Context, msg: Message)
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

    async fn interaction_create(&self, ctx: serenity::Context, interaction: serenity::Interaction)
    {
        if let serenity::Interaction::Command(command) = &interaction
        {
            let cmd_str = command.data.name.as_str();
            match cmd_str
            {
               "search" => special_interaction(ctx, &interaction).await,
               "ai" => special_interaction(ctx, &interaction).await,
               "password" => special_interaction(ctx, &interaction).await,
                _ => {}
            };
        }
    }

    async fn ready(&self, ctx: serenity::Context, ready: serenity::Ready)
    {
        println!("{} is connected!", ready.user.name);
        let channel_id = serenity::ChannelId::new(
                    env::var("MAIN_CHANNEL_ID")
                    .expect("Expected MAIN_CHANNEL_ID in environment")
                    .parse()
                    .expect("MAIN_CHANNEL_ID must be an integer"));
        begin_scheduled_jobs(channel_id, ctx);
    }
}

async fn begin_scheduled_jobs(channel_id: serenity::ChannelId, ctx: serenity::Context) -> Result<(), Error>
{
    let trump_inauguration_date: DateTime<Utc> = DateTime::parse_from_rfc2822("Mon, 20 Jan 2024 12:00:00 -0500").unwrap().to_utc();
    let schedule = JobScheduler::new().await?;
    let ctx_clone = ctx.clone();
    schedule.add(
        Job::new("0 0 14 * *  Fri", move |_uuid, _l| // 2PM UTC => 9AM EST
        {
            let http = &ctx_clone.http.clone();
            let rt = tokio::runtime::Runtime::new().unwrap();
            let builder = serenity::CreateMessage::new().content("https://www.youtube.com/watch?v=WUyJ6N6FD9Q");
            let future = channel_id.send_message(http, builder);
            let _ = rt.block_on(future);
        })?);
    schedule.add(
        Job::new("1 * * * * *", move |_uuid, _l|
        {
            let current_time = chrono::offset::Local::now().to_utc();
            let td = current_time - trump_inauguration_date;
            let data = breakdown_time(td);
            let activity_string = format!("{0} weeks, {1} days, {2} hours, and {3} seconds until Trump's presidency is over", data.weeks, data.days, data.hours, data.seconds);
            ctx.set_activity(Some(poise::serenity_prelude::ActivityData::custom(activity_string)));
        })?);
    schedule.start().await?;
    return Ok(());
}

fn breakdown_time(td: TimeDelta) -> TimeData
{
    let mut seconds = td.num_seconds();
    let years = seconds % YEAR_IN_SECONDS as i64;
    seconds -= YEAR_IN_SECONDS as i64 * years;
    let months = seconds % MONTH_IN_SECONDS as i64;
    seconds -= MONTH_IN_SECONDS as i64 * months;
    let weeks = seconds % WEEK_IN_SECONDS as i64;
    seconds -= WEEK_IN_SECONDS as i64 * weeks;
    let days = seconds % DAY_IN_SECONDS as i64;
    seconds -= DAY_IN_SECONDS as i64 * days;
    let hours = seconds % HOUR_IN_SECONDS as i64;
    seconds -= HOUR_IN_SECONDS as i64 * hours;
    let minutes = seconds % MIN_IN_SECONDS as i64;
    seconds -= MIN_IN_SECONDS as i64 * minutes;
    TimeData
    {
        years,
        months,
        weeks,
        days,
        hours,
        minutes,
        seconds,
    }
}

//handle interactions that require doing some extra stuff other than just sending to the channel
async fn special_interaction(ctx: serenity::Context, interaction: &Interaction)
{
    if let Interaction::Command(command) = &interaction
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
//           "ai" => commands::ai::interaction(ctx, command).await,
           "password" => commands::password::interaction(ctx, command).await,
           &_ => println!("Unimplemented"),
        }
    }
}

#[allow(deprecated)]
async fn no_results(ctx: serenity::Context, command: &ApplicationCommandInteraction)
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
//TODO: add test region so I can test function output without actually connecting to discord
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
