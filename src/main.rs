extern crate rand;

use crate::handler::VoiceManager;
use handler::Handler;
use serenity::Client;
use std::env;
use std::sync::Arc;

mod commands;
mod files;
mod handler;

fn main() {
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set.");

    let mut client = Client::new(&token, Handler).expect("Error creating client.");

    {
        // Obtain a lock to the data owned by the client, and insert the client's
        // voice manager into it. This allows the voice manager to be accessible by
        // event handlers and framework commands.
        let mut data = client.data.write();
        data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
    }

    if let Err(why) = client.start() {
        println!("An error occurred while starting the client: {:?}", why);
    }
}
