use serenity::model::mention::Mention;
use poise::serenity_prelude::GuildChannel;

use crate::{Context, Error, Message};

extern crate dotenv;
extern crate serde_json;

#[poise::command(slash_command, rename = "ping_voice")]
pub async fn run(
    ctx: Context<'_>,
    #[description = "The channel to address"] channel: GuildChannel,
    #[description = "Optional message to attach with the ping"] message: Option<String>)
    -> Result<(), Error>
{
        let id = channel.id;
        let to_channel = id.to_channel(&ctx.http()).await.unwrap();
        let guild = to_channel.guild().unwrap();
        let mut s: String = String::from("");
        if let Ok(members) = guild.members(ctx.cache())
        {
            //mention all of the users in the channel
            for member in members {
                s.push_str(Mention::from(member.user.id).to_string().as_str());
            }
            //check if the user attached a message with the command
            match message
            {
                Some(m) =>
                {
                    s.push_str(m.as_str());
                },
                None => {},
            }
            let _ = ctx.say(s).await;
        }
        else
        {
            let _ = ctx.say("No one detected in the voice channel").await;
        }
        Ok(())
}
