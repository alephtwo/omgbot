use crate::{commands, files};
use serenity::{
    client::bridge::voice::ClientVoiceManager,
    model::{
        gateway::Ready,
        id::ChannelId,
        prelude::{Guild, Message},
        Permissions,
    },
    prelude::{Context, EventHandler, Mutex, RwLock, TypeMapKey},
    voice,
};
use std::{fs::File, iter::FromIterator, path::PathBuf, sync::Arc, thread, time::Duration};
use crate::commands::Command::{Help, PlaySound};

pub struct Handler;
pub struct VoiceManager;

impl TypeMapKey for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        match commands::parse_command(&msg).unwrap() {
            Help => post_help(ctx, msg),
            PlaySound(category) => play_sound(ctx, msg, category)
        };
    }

    fn ready(&self, ctx: Context, payload: Ready) {
        println!("[{}] connected to Discord.", payload.user.name);
        let url = payload
            .user
            .invite_url(&ctx.http, Permissions::empty())
            .unwrap();

        println!("Invite URL: {}", url);
    }
}

fn post_help(ctx: Context, msg: Message) {
    let mut commands: Vec<String> = Vec::from_iter(commands::commands().iter().cloned());
    commands.sort();

    let lines = commands
        .iter()
        .map(|c| format!("* `!{}`", c))
        .collect::<Vec<String>>()
        .join("\n");

    if let Err(why) = msg.channel_id.say(&ctx.http, format!("Commands:\n{}", lines)) {
        eprintln!("Error sending message: {:?}", why);
    }
}

fn play_sound(ctx: Context, msg: Message, category: String) {
    // We will need to know what guild we're in.
    let guild = msg.guild(&ctx.cache).unwrap();

    // Let's pick a file to play.
    let source = files::pick_file(category).unwrap();

    // What voice channel is this user in?
    let channel = match get_voice_channel_for_user(&guild, &msg) {
        Some(c) => c,
        None => {
            // They're not in a channel, so we should upload the file instead.
            upload_sound(&ctx, &msg, &source);
            return;
        }
    };

    // Grab the voice manager from the cache.
    let voice_manager = get_voice_manager_from_cache(&ctx).unwrap();

    // Convert the source to an AudioSource so it can be played.
    let audio_source = voice::ffmpeg(&source).unwrap();

    // We know the user's in a voice channel. Let's join it and play the sound...
    let guild_id = guild.read().id;
    let mut manager = voice_manager.lock();
    // Get a handle to the audio.
    let audio_lock = manager
        .join(guild_id, channel)
        .map(|handler| {
            // Sometimes, things can move a little quickly.
            // The sound might play while the connected "ping" is still playing.
            // Sleep for just a little bit before playing.
            thread::sleep(Duration::from_secs(1));
            handler.play_only(audio_source)
        })
        .unwrap();

    // Poll until we're done.
    thread::spawn(move || {
        loop {
            if audio_lock.lock().finished {
                break;
            }
            thread::sleep(Duration::from_millis(500));
        }
        let vm = get_voice_manager_from_cache(&ctx).unwrap();
        vm.lock().leave(guild_id);
        // This currently leaves a zombie, which is something we should address...
    });
}

fn upload_sound(ctx: &Context, msg: &Message, source: &PathBuf) {
    let file = File::open(source).unwrap();
    let name = source.file_name().unwrap().to_str().unwrap();

    msg.channel_id
        .send_files(&ctx.http, vec![(&file, name)], |m| {
            m.content("You aren't in a channel, but here's the sound...");
            m
        })
        .ok();
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
