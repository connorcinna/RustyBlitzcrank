use rand::Rng;
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn run(
    ctx: Context<'_>,
    #[description = "Ceiling for the random number generation"] ceiling: i64,
    ) -> Result<(), Error>
{
    let mut rng = rand::rng();
    let roll_result: i64 = rng.random_range(0..ceiling);
    ctx.say(format!("{}", roll_result));
    Ok(())
}
