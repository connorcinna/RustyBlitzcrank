use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn run(ctx: Context<'_>) -> Result<(), Error> 
{
    ctx.say("https://github.com/connorcinna/RustyBlitzcrank#readme");
    Ok(())
}
