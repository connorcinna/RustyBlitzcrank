use crate::Context;
use poise::serenity_prelude as serenity;
use serenity::Message;

pub enum Website
{
    Twitter,
    X,
    Tiktok,
    Instagram,
    Reddit,
}

pub struct LinkFix<'a>
{
    pub website: Website,
    pub old_link: &'a str,
    pub new_link: &'a str,
}

pub async fn fix_links<'a>(old_link: &'a str, new_link: &'a str, msg: &'a Message, ctx: Context<'a>)
{
    let mut final_link = msg.content.to_owned();
    let prepend_str = format!("Posted by {}\n", msg.author.name);
    final_link = final_link.replace(&old_link, &new_link);
    final_link.insert_str(0, &prepend_str);
    msg.channel_id.say(&ctx.http(), final_link).await.unwrap();
    msg.delete(&ctx.http()).await.expect("Unable to delete message");
}
