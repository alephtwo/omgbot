use serenity::model::prelude::Message;
use serenity::prelude::{Context, EventHandler};
use serenity::model::gateway::Ready;
use serenity::model::Permissions;
use std::collections::HashSet;

pub struct Handler;

const PREFIX: &str = "!";

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        // Only allow valid commands to get through.
        let command = match parse_command(&msg) {
            Some(t) => t,
            None => return
        };

        // TODO: Make this play the sound.
        // For now, just report that we parsed a command.
        msg.channel_id.say(&ctx.http, command).expect("Error");
    }

    fn ready(&self, ctx: Context, payload: Ready) {
        println!("[{}] connected to Discord.", payload.user.name);
        let url = match payload.user.invite_url(&ctx.http, Permissions::empty()) {
            Ok(v) => v,
            Err(why) => {
                eprintln!("Error getting invite url: {:?}", why);
                return;
            }
        };

        println!("Invite URL: {}", url);
    }
}

fn parse_command(msg: &Message) -> Option<String> {
    let content = &msg.content;

    // If the message doesn't start with the prefix it's not a command. Stop.
    if !content.starts_with(PREFIX) {
        return None;
    }

    // If the message contains more than one token, it's not a command. Stop.
    if content.split_whitespace().count() > 1 {
        return None;
    }

    // Now we know it's a command, or at least an attempt at one. Let's grab it.
    let command: &str = &content.replace(PREFIX, "");

    // If it's not a valid command, we should stop.
    if !get_command_set().contains(&command) {
        return None;
    }

    // It's a valid command!
    Some(command.to_string())
}

fn get_command_set() -> HashSet<&'static str> {
    let mut set = HashSet::new();
    set.insert("clarisse");
    set.insert("grats");
    set.insert("grimnir");
    set.insert("jewels");
    set.insert("kaine");
    set.insert("medusa");
    set.insert("omg");
    set.insert("robot");
    set.insert("thunder");
    set.insert("ugaa");
    set
}
