use rand::Rng;
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn run(
    ctx: Context<'_>,
    #[description = "Ceiling for the random number generation"] ceiling: i64,
    ) -> Result<(), Error>
{
    //rand::rng does not implement either Send or Sync,
    //so in this case, random needs to fall out of scope before we call await
    let roll =
    {
        let mut random = rand::rng();
        random.random_range(0..ceiling)
    };
    let _ = ctx.say(format!("{}", roll)).await;
    Ok(())
}
