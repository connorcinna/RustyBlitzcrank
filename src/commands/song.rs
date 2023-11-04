use std::println;

use rspotify::model::SearchResult;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::{CommandDataOption, CommandDataOptionValue};
use rspotify::{prelude::*, ClientCredsSpotify, Credentials};

pub async fn run(options: &[CommandDataOption]) -> String {
    let option = options
            .get(0)
            .expect("Expected query option")
            .resolved
            .as_ref()
            .expect("Expected query option");

    if let CommandDataOptionValue::String(query) = option {
    	let creds = Credentials::from_env().unwrap();
    	println!("Credentials ID: {}, Credentials Secret: {:?}", creds.id, creds.secret);
		let spotify = ClientCredsSpotify::new(creds);
		spotify.request_token().await.unwrap();
		let song = spotify.search(
			query,
			rspotify::model::SearchType::Track,
			None,
			None,
			None,
			None).await;
		let song_string= match song {
			Ok(res) => res,
			Err(e) => panic!("Error searching for song: {:?}", e), 
		};
		if let SearchResult::Tracks(page) = song_string {
			if let Some(track) = page.items[0].external_urls.get("spotify") {
				return track.to_string();
			}
			else { 
				return String::from("Unable to find track, try including the artist's name");
			}
		}
    }
    String::from("Unable to find track for unknown reasons because @isthistheblood is FUCKING STUPID")

}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("song").description("Search Spotify API for a song")
    .create_option(|option| {
        option
            .name("word")
            .description("Search Spotify API for a song")
            .kind(CommandOptionType::String)
            .required(true)
        });
    return command;
} 