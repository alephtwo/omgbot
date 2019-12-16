use handler::Handler;
use serenity::Client;
use std::env;

mod handler;

fn main() {
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set.");

    let mut client = Client::new(&token, Handler).expect("Error creating client.");

    if let Err(why) = client.start() {
        println!("An error occurred while starting the client: {:?}", why);
    }
}
