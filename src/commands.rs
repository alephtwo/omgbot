use serenity::model::prelude::Message;
use std::collections::HashSet;

const PREFIX: &str = "!";

pub fn parse_command(msg: &Message) -> Option<String> {
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
    if !commands().contains(&command) {
        return None;
    }

    // It's a valid command!
    Some(command.to_string())
}

fn commands() -> HashSet<&'static str> {
    let mut set = HashSet::new();
    set.insert("clarisse");
    set.insert("grats");
    set.insert("grimnir");
    set.insert("jewels");
    set.insert("kaine");
    set.insert("medusa");
    set.insert("michiru");
    set.insert("omg");
    set.insert("robot");
    set.insert("ruria");
    set.insert("thunder");
    set.insert("ugaa");
    set
}
