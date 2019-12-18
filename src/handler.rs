use serenity::{
    client::bridge::voice::ClientVoiceManager,
    model::{
        gateway::Ready,
        id::ChannelId,
        prelude::{Guild, Message},
        Permissions,
    },
    prelude::{Context, EventHandler, Mutex, RwLock, TypeMapKey},
    voice::{ffmpeg, AudioSource},
};
use std::{collections::HashSet, sync::Arc, thread, time::Duration};

const PREFIX: &str = "!";

pub struct Handler;
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

        // What voice channel is this user in?
        let channel = match get_voice_channel_for_user(&guild, &msg) {
            Some(c) => c,
            None => {
                eprintln!(
                    "User {} issued command {} but is not in a voice channel.",
                    msg.author.name, command
                );
                return;
            }
        };

        // Grab the voice manager from the cache.
        let voice_manager = match get_voice_manager_from_cache(&ctx) {
            Some(vm) => vm,
            None => {
                eprintln!("No voice manager in cache.");
                return;
            }
        };

        // Let's pick a file to play.
        let source = match pick_file(command) {
            Some(s) => s,
            None => return,
        };

        // We know the user's in a voice channel. Let's join it and play the sound...
        let guild_id = guild.read().id;
        let mut manager = voice_manager.lock();
        // Get a handle to the audio.
        let audio_lock = match manager.join(guild_id, channel) {
            Some(handler) => handler.play_returning(source),
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

fn get_voice_channel_for_user(guild: &Arc<RwLock<Guild>>, msg: &Message) -> Option<ChannelId> {
    guild
        .read()
        .voice_states
        .get(&msg.author.id)
        .and_then(|vs| vs.channel_id)
}

fn get_voice_manager_from_cache(ctx: &Context) -> Option<Arc<Mutex<ClientVoiceManager>>> {
    ctx.data.read().get::<VoiceManager>().cloned()
}

fn pick_file(_category: String) -> Option<Box<dyn AudioSource>> {
    // TODO: Hardcode this for now; ideally we will find a random file in the category.
    match ffmpeg("./sounds/omg/rlm-01.mp3") {
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
