use crate::commands::Command::{Help, PlaySound};
use crate::files;
use serenity::model::prelude::Message;
use std::{collections::HashSet, fs, iter::FromIterator};

const PREFIX: &str = "!";

pub enum Command {
    Help,
    PlaySound(String),
}

pub fn parse_command(msg: &Message) -> Option<Command> {
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

    if command == "help" {
        return Some(Help);
    }

    // If it's not a valid command, we should stop.
    if commands().contains(command) {
        return Some(PlaySound(command.to_string()));
    }

    // It's a valid command!
    return None;
}

pub fn commands() -> HashSet<String> {
    let paths: Vec<String> = fs::read_dir(files::sound_dir())
        .unwrap()
        .map(|p| {
            p.unwrap()
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned()
        })
        .collect();

    HashSet::from_iter(paths.iter().cloned())
}
