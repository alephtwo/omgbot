use crate::{commands, sound_picker};
use serenity::{
    client::bridge::voice::ClientVoiceManager,
    model::{
        gateway::Ready,
        id::ChannelId,
        prelude::{Guild, Message},
        Permissions,
    },
    prelude::{Context, EventHandler, Mutex, RwLock, TypeMapKey},
};
use std::{sync::Arc, thread, time::Duration, path::PathBuf};
use std::fs::File;

pub struct Handler;
pub struct VoiceManager;

impl TypeMapKey for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        // Only allow valid commands to get through.
        let command = match commands::parse_command(&msg) {
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

        // Let's pick a file to play.
        let source = match sound_picker::pick_file(command) {
            Some(s) => s,
            None => {
                eprintln!("No sounds in path.");
                return;
            }
        };

        // What voice channel is this user in?
        let channel = match get_voice_channel_for_user(&guild, &msg) {
            Some(c) => c,
            None => {
                upload_sound(&ctx, &msg, &source);
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

        // Convert the source to an AudioSource so it can be played.
        let audio_source = match sound_picker::to_audio_source(&source) {
            Some(s) => s,
            None => {
                eprintln!("Audio could not be converted to an AudioSource.");
                return;
            }
        };

        // We know the user's in a voice channel. Let's join it and play the sound...
        let guild_id = guild.read().id;
        let mut manager = voice_manager.lock();
        // Get a handle to the audio.
        let audio_lock = match manager.join(guild_id, channel) {
            Some(handler) => {
                // Sometimes, things can move a little quickly.
                // The sound might play while the connected "ping" is still playing.
                // Sleep for just a little bit before playing.
                thread::sleep(Duration::from_secs(1));
                handler.play_only(audio_source)
            }
            None => {
                eprintln!("Unable to get a handler for the voice channel.");
                return;
            }
        };

        // Poll until we're done.
        thread::spawn(move || {
            loop {
                if audio_lock.lock().finished {
                    break;
                }
                thread::sleep(Duration::from_millis(500));
            }
            let vm = match get_voice_manager_from_cache(&ctx) {
                Some(vm) => vm,
                None => {
                    eprintln!("No voice manager in cache.");
                    return;
                }
            };
            vm.lock().leave(guild_id);
            // This currently leaves a zombie, which is something we should address...
        });
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

fn upload_sound(ctx: &Context, msg: &Message, source: &PathBuf) {
    let file = match File::open(source) {
        Ok(file) => file,
        Err(why) => {
            eprintln!("Error opening file: {:?}", why);
            return;
        }
    };

    let name = source.file_name().unwrap().to_str().unwrap();

    let result = msg.channel_id.send_files(&ctx.http, vec![(&file, name)], |m| {
        m.content("You aren't in a channel, but here's the sound...");
        m
    });
    match result {
        Ok(_a) => {},
        Err(why) => {
            eprintln!("Unable to upload sound: {:?}", why);
        }
    };
}
