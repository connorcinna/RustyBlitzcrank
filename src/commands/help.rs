use crate::{Context, Error};

#[poise::command(slash_command, rename = "help")]
pub async fn run(ctx: Context<'_>) -> Result<(), Error>
{
    let _ = ctx.say("https://github.com/connorcinna/RustyBlitzcrank#readme").await;
    Ok(())
}
