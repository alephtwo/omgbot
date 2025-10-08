use crate::BotConfig;
use glob::glob;
use rand::{rng, seq::IteratorRandom};
use serenity::{
    all::{ChannelId, Context, CreateAttachment, CreateMessage, GuildId, Message, MessageBuilder},
    async_trait,
};
use songbird::{Event, EventHandler, TrackEvent, input::File};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Clone)]
pub struct LeaveAfterPlaying {
    pub manager: Arc<songbird::Songbird>,
    pub guild: serenity::model::id::GuildId,
}

#[async_trait]
impl EventHandler for LeaveAfterPlaying {
    async fn act(&self, _ctx: &songbird::events::EventContext<'_>) -> Option<Event> {
        let _ = self.manager.leave(self.guild).await;
        None
    }
}

pub async fn play_sound_in_response_to(
    ctx: Context,
    msg: Message,
    file: PathBuf,
    config: &BotConfig,
) {
    // Make sure this command came from a guild
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            eprintln!("Not in a guild");
            return;
        }
    };

    let guild = match msg.guild(&ctx.cache) {
        Some(guild) => guild.clone(),
        None => {
            eprintln!("Guild not in cache");
            return;
        }
    };

    let voice_channel = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|vs| vs.channel_id);

    match voice_channel {
        Some(channel_id) => {
            println!("{} requested {:?}", msg.author.name, file);
            // They are in a voice channel, so play the sound there.
            play_sound(ctx, guild_id, channel_id, file, config).await
        }
        None => {
            // Try to open the file for reading
            let opened_file: tokio::fs::File = match tokio::fs::File::open(&file).await {
                Ok(f) => f,
                Err(err) => {
                    eprintln!("Failed to open sound file: {err}");
                    let _ = msg.reply(&ctx.http, "Failed to open the sound file.").await;
                    return;
                }
            };

            // They are NOT in a voice channel. Upload the sound to the
            // text channel they posted in
            let attachment = CreateAttachment::file(
                &opened_file,
                file.file_name()
                    .expect("nameless file")
                    .to_str()
                    .expect("non unicode filename"),
            )
            .await
            .expect("failed to upload attachment");
            let content = MessageBuilder::new().build();
            let message = CreateMessage::new()
                .add_file(attachment)
                .content(content)
                .reference_message(&msg);

            // Send message + file
            let result = msg.channel_id.send_message(&ctx.http, message).await;
            if let Err(err) = result {
                eprintln!("Error responding to command: {err}");
            }
        }
    }
}

pub async fn play_sound(
    ctx: Context,
    guild_id: GuildId,
    channel_id: ChannelId,
    file: PathBuf,
    config: &BotConfig,
) {
    // Join that channel.
    let manager = songbird::get(&ctx)
        .await
        .expect("Failed to instantiate songbird")
        .clone();

    let handler_lock = match manager.join(guild_id, channel_id).await {
        Ok(lock) => lock,
        Err(e) => {
            eprintln!("{e:?}");
            return;
        }
    };

    let mut handler = handler_lock.lock().await;
    let track_handle = handler.play_only(File::new(file).into());

    // please don't break everyone's eardrums
    track_handle
        .set_volume(config.volume)
        .expect("I am so sorry. I destroyed everyone's ears.");

    let _ = track_handle.add_event(
        Event::Track(TrackEvent::End),
        LeaveAfterPlaying {
            manager: manager.clone(),
            guild: guild_id,
        },
    );
}

pub fn choose_sound(soundbank: &Path, category: String) -> PathBuf {
    let source_dir = soundbank.join(category);
    let children = list_children(source_dir.as_path()).filter(|f| f.is_file());
    children.choose(&mut rng()).expect("no children")
}

pub fn choose_any_sound(soundbank: &Path) -> PathBuf {
    let pattern = soundbank.join("**/*");
    let pattern_str = pattern.to_str().expect("Non-UTF8 path not supported");

    let sounds = glob(pattern_str)
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .filter(|f| f.is_file());

    sounds.choose(&mut rng()).expect("no children")
}

pub fn list_categories(soundbank: &Path) -> impl Iterator<Item = String> {
    list_category_directories(soundbank).filter_map(|f| get_category_name(&f))
}

pub fn list_category_directories(soundbank: &Path) -> impl Iterator<Item = PathBuf> {
    list_children(soundbank).filter(|path| path.is_dir()) // only directories
}

pub fn list_children(path: &Path) -> impl Iterator<Item = PathBuf> {
    let pattern = path.join("*");
    let pattern_str = pattern.to_str().expect("Non-UTF8 path not supported");

    glob(pattern_str)
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok) // skip invalid entries
}

pub fn get_category_name(path: &Path) -> Option<String> {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|s| s.to_string())
}
