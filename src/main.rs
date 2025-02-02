mod commands;
mod common;
mod websites;
extern crate dotenv;

use std::env;
use dotenv::dotenv;
use tokio_cron_scheduler::{Job, JobScheduler};
use chrono::{DateTime, TimeDelta};
use poise::serenity_prelude as serenity;
use serenity::Message;
use serenity::model::gateway::{GatewayIntents, Ready};

use crate::websites::{fix_links, LINKS};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

struct Data {}
//TODO move all this extra date and struct stuff somewhere else
const YEAR_IN_SECONDS: i64 = 60 * 60 * 24 * 365;
const MONTH_IN_SECONDS: i64 = YEAR_IN_SECONDS / 12;
const WEEK_IN_SECONDS: i64 = MONTH_IN_SECONDS / 4;
const DAY_IN_SECONDS: i64 = WEEK_IN_SECONDS / 7;
const HOUR_IN_SECONDS: i64 = DAY_IN_SECONDS / 24;
const MIN_IN_SECONDS: i64 = HOUR_IN_SECONDS / 60;

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


async fn begin_scheduled_jobs(channel_id: serenity::ChannelId, ctx: serenity::Context) -> Result<(), Error>
{
    let trumps_last_day = DateTime::parse_from_rfc2822("Sat, 20 Jan 2029 12:00:00 -0500").unwrap().to_utc();
    let schedule = JobScheduler::new().await?;
    let ctx_clone = ctx.clone();
    let _ = schedule.add(
        Job::new("0 0 14 * *  Fri", move |_uuid, _l| // 2PM UTC => 9AM EST
        {
            let http = &ctx_clone.http.clone();
            let rt = tokio::runtime::Runtime::new().unwrap();
            let builder = serenity::CreateMessage::new().content("https://www.youtube.com/watch?v=WUyJ6N6FD9Q");
            let future = channel_id.send_message(http, builder);
            let _ = rt.block_on(future);
        })?).await;
    //run every second
    let _ = schedule.add(
        Job::new("* * * * * *", move |_uuid, _l|
        {
            let current_time = chrono::offset::Local::now().to_utc();
            let td = trumps_last_day - current_time;
            let data = breakdown_time(td);
            let activity_string = format!("{0} years, {1} months, {2} weeks, {3} days, {4} hours, {5} minutes, and {6} seconds until Trump's presidency is over",
                data.years,
                data.months,
                data.weeks,
                data.days,
                data.hours,
                data.minutes,
                data.seconds);
            ctx.set_activity(Some(poise::serenity_prelude::ActivityData::custom(activity_string)));
        })?).await;
    schedule.start().await?;
    Ok(())
}

//TODO: put this in Utility file
fn breakdown_time(td: TimeDelta) -> TimeData
{
    let mut seconds = td.num_seconds();
    let years = seconds / YEAR_IN_SECONDS;
    seconds -= YEAR_IN_SECONDS * years;
    let months = seconds / MONTH_IN_SECONDS;
    seconds -= MONTH_IN_SECONDS * months;
    let weeks = seconds / WEEK_IN_SECONDS;
    seconds -= WEEK_IN_SECONDS * weeks;
    let days = seconds / DAY_IN_SECONDS;
    seconds -= DAY_IN_SECONDS * days;
    let hours = seconds / HOUR_IN_SECONDS;
    seconds -= HOUR_IN_SECONDS * hours;
    let minutes = seconds / MIN_IN_SECONDS;
    seconds -= MIN_IN_SECONDS * minutes;
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

#[poise::command(prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

async fn event_handler
(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error>
{
    match event
    {
        serenity::FullEvent::Ready { data_about_bot, .. } =>
        {
            ready(ctx, data_about_bot).await;
        }
        serenity::FullEvent::Message { new_message } =>
        {
            message(ctx, new_message).await;
        }
        _ => {}
    }
    Ok(())
}

async fn message(ctx: &serenity::Context, msg: &Message)
    {
        for link in LINKS
        {
            if msg.content.find(&link.old_link).is_some()
            {
                println!("fixing link: {0}", link.old_link);
                fix_links(link.old_link, link.new_link, &msg, ctx.clone()).await;
            }
        }
    }

async fn ready(ctx: &serenity::Context, ready: &Ready)
{
    println!("{} is connected!", ready.user.name);
    let channel_id = serenity::ChannelId::new(
                env::var("MAIN_CHANNEL_ID")
                .expect("Expected MAIN_CHANNEL_ID in environment")
                .parse()
                .expect("MAIN_CHANNEL_ID must be an integer"));
    let _ = begin_scheduled_jobs(channel_id, ctx.clone()).await;
}

//TODO: add test region so I can test function output without actually connecting to discord
#[tokio::main]
async fn main()
{
    dotenv().ok();
    let token = env::var("CLIENT_TOKEN").expect("Expected a token in the environment");
    let intents = serenity::GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGES;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions
        {
            commands: vec!
            [
                register(),
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
            event_handler: |ctx, event, framework, data|
            {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            on_error: |error|
            {
                Box::pin(async move
                {
                    match error
                    {
                        poise::FrameworkError::ArgumentParse {  .. } =>
                        {
                            println!("Error parsing arguments to Poise framework builder");
                        }
                        other => poise::builtins::on_error(other).await.unwrap(),
                    }

                })
            },
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework|
        {
            Box::pin(async move
            {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                    Ok(Data{})
            })
        })
        .build();
    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
