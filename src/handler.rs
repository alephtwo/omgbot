use serenity::client::bridge::voice::ClientVoiceManager;
use serenity::model::gateway::Ready;
use serenity::model::prelude::Message;
use serenity::model::Permissions;
use serenity::prelude::{Context, EventHandler, Mutex, TypeMapKey};
use std::collections::HashSet;
use std::sync::Arc;
use serenity::voice;
use serenity::voice::AudioSource;
use std::time::Duration;
use std::thread::sleep;

pub struct Handler;

const PREFIX: &str = "!";

pub struct VoiceManager;
impl TypeMapKey for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        // Only allow valid commands to get through.
        let command = match parse_command(&msg) {
            Some(t) => t,
            None => return,
        };

        // We will need to know what guild we're in.
        let guild = match msg.guild(&ctx.cache) {
            Some(guild) => guild,
            None => {
                eprintln!("Groups and DMs are not supported.");
                return;
            }
        };

        let guild_id = guild.read().id;

        // What voice channel is this user in?
        let channel = match guild
            .read()
            .voice_states
            .get(&msg.author.id)
            .and_then(|vs| vs.channel_id)
        {
            Some(c) => c,
            None => {
                eprintln!(
                    "User {} issued command {} but is not in a voice channel.",
                    msg.author.name, command
                );
                return;
            }
        };

        // Let's pick a file to play.
        let source = match pick_file(command) {
            Some(s) => s,
            None => return
        };

        // We know the user's in a voice channel. Let's join it and play the sound...
        let manager_lock = ctx
            .data
            .read()
            .get::<VoiceManager>()
            .cloned()
            .expect("Expected VoiceManager in ShareMap.");
        let mut manager = manager_lock.lock();
        match manager.join(guild_id, channel) {
            Some(handler) => {
                handler.play(source)
            },
            None => {
                eprintln!("Unable to get a handler for the voice channel.");
                return;
            }
        };
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

fn pick_file(_category: String) -> Option<Box<dyn AudioSource>> {
    // TODO: Rickroll, for now.
    match voice::ffmpeg("./sounds/omg/rlm-01.mp3") {
        Ok(source) => Some(source),
        Err(why) => {
            eprintln!("Error picking source: {:?}", why);
            return None;
        }
    }
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
