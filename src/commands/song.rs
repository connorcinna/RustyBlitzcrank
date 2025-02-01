use rspotify::{prelude::*, ClientCredsSpotify, Credentials};
use rspotify::model::SearchResult;
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn run(
    ctx: Context<'_>,
    #[description = "The query to be passed to Spotify's API"] query: String
) -> Result<(), Error> {

    let creds = Credentials::from_env().unwrap();
    let spotify = ClientCredsSpotify::new(creds);
    spotify.request_token().await.unwrap();
    let song = spotify.search(
        &query,
        rspotify::model::SearchType::Track,
        None,
        None,
        None,
        None)
        .await
        .expect("Expected SearchResult API response from Spotify");
    if let SearchResult::Tracks(page) = song
    {
        if let Some(track) = page.items[0].external_urls.get("spotify")
        {
            let _ = ctx.say(track.to_string()).await;
            return Ok(())
        }
        else
        {
            return Err("Unable to find track, try including the artist's name".into());
        }
    }
    let _ = ctx.say("Unable to find track for unknown reasons").await;
    Err("Unable to find track for unknown reasons".into())
}
