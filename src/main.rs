use std::env;

use serenity::Client;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::model::channel::Message;
use serenity::prelude::{Context, EventHandler};

fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("DISCORD_TOKEN must be set.");

    let mut client = Client::new(&token, Handler)
        .expect("Error creating client.");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    client.with_framework(framework);

    if let Err(why) = client.start () {
        println!("An error occurred while starting the client: {:?}", why);
    }
}

struct Handler;
impl EventHandler for Handler {}

group!({
    name: "general",
    options: {},
    commands: [ping]
});

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "pong").expect("unable to pong");
    Ok(())
}
