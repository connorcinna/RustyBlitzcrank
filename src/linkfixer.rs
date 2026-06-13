use poise::serenity_prelude as serenity;
use serenity::Message;

pub const LINKS : [LinkFix; 5] =
[
    LinkFix {website: Website::Twitter, old_link: "https://twitter.com", new_link: "https://vxtwitter.com"},
    LinkFix {website: Website::X, old_link: "https://x.com", new_link: "https://c.vxtwitter.com"},
    LinkFix {website: Website::Tiktok, old_link: "https://www.tiktok.com", new_link: "https://vxtiktok.com"},
    LinkFix {website: Website::Instagram, old_link: "https://www.instagram.com", new_link: "https://instagramez.com"},
    LinkFix {website: Website::Reddit, old_link: "https://www.reddit.com", new_link: "https://vxreddit.com"},
];

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

pub async fn fix_links<'a>(old_link: &'a str, new_link: &'a str, msg: &'a Message, ctx: serenity::Context)
{
    let mut final_link = msg.content.to_owned();
    let prepend_str = format!("Posted by {}\n", msg.author.name);
    final_link = final_link.replace(&old_link, &new_link);
    final_link.insert_str(0, &prepend_str);
    msg.channel_id.say(&ctx.http, final_link).await.unwrap();
    msg.delete(&ctx.http).await.expect("Unable to delete message");
}
