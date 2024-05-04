pub enum Website
{
    Twitter,
    X,
    Tiktok,
    Instagram,
    Reddit,
}

pub struct LinkFix
{
    pub website: Website,
    pub old_link: String,
    pub new_link: String,
}
