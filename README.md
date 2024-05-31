# RustyBlitzcrank

A Discord bot written in Rust using https://github.com/serenity-rs/serenity.

![RustyBlitzcrank logo](https://github.com/connorcinna/RustyBlitzcrank/blob/master/resources/rb.webp)

## Commands
-`/gif [word]` Queries the Tenor API for a keyword and prints the first gif returned.  
-`/vid [word]` Queries the Youtube search API and prints the first video returned.  
-`/search [word]` Queries a custom Google search and prints the first image returned.  
-`/song [word]` Queries the Spotify API and embeds the first song returned.  
-`/roll [number]` Returns a random number between 1 and the argument provided.  
-`/name` Returns a random name made of two random words (see resources/words.json) and padded with numbers if the words combined are less than 16 characters in length.  
-`/ai [prompt]` Sends a prompt to llama3 and returns the response.  
-`/password [number]` Generates a random password of the length provided. Includes numbers, letters, special characters and variable casing.  
-`/ping_voice [channel name]` Ping everyone in the provided voice channel.  
-`/jerma` :)   
