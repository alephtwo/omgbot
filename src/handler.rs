use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::model::prelude::Message;
use serenity::model::Permissions;
use serenity::prelude::{Context, EventHandler};
use std::collections::HashSet;

pub struct Handler;

const PREFIX: &str = "!";

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        // Only allow valid commands to get through.
        let command = match parse_command(&msg) {
            Some(t) => t,
            None => return,
        };

        // What voice channel is the user that requested this sound in?
        let channel = match user_voice_channel(&ctx, &msg) {
            Some(c) => c,
            None => {
                eprintln!(
                    "User {} issued command {} but is not in a voice channel.",
                    msg.author.name, command
                );
                return;
            }
        };

        // We know the user's in a channel. Let's join it and play a sound.
        play_sound(channel, command);
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

fn user_voice_channel(ctx: &Context, msg: &Message) -> Option<ChannelId> {
    let guild = match msg.guild(&ctx.cache) {
        Some(guild) => guild,
        None => {
            eprintln!("Groups and DMs are not supported.");
            return None;
        }
    };

    let channel_id = guild
        .read()
        .voice_states
        .get(&msg.author.id)
        .and_then(|vs| vs.channel_id);

    channel_id
}

fn play_sound(channel: ChannelId, category: String) {
    println!("Playing sound (category: {})...", category);
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
