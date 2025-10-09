use crate::BotConfig;
use anyhow::{anyhow, bail};
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
) -> Result<(), anyhow::Error> {
    // Make sure this command came from a guild
    let guild_id = msg.guild_id.ok_or(anyhow!("Not in a guild"))?;
    let guild = msg
        .guild(&ctx.cache)
        .ok_or(anyhow!("Guild not in cache"))?
        .clone();

    let voice_channel = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|vs| vs.channel_id);

    match voice_channel {
        Some(channel_id) => {
            tracing::info!("{} requested {:?}", msg.author.name, file);
            // They are in a voice channel, so play the sound there.
            play_sound(ctx, guild_id, channel_id, file, config).await
        }
        None => {
            // Try to open the file for reading
            let opened_file = tokio::fs::File::open(&file).await?;

            // They are NOT in a voice channel. Upload the sound to the
            // text channel they posted in
            let attachment = CreateAttachment::file(
                &opened_file,
                file.file_name()
                    .ok_or(anyhow!("nameless file"))?
                    .to_str()
                    .ok_or(anyhow!("non unicode filename"))?,
            )
            .await?;
            let content = MessageBuilder::new().build();
            let message = CreateMessage::new()
                .add_file(attachment)
                .content(content)
                .reference_message(&msg);

            // Send message + file
            let result = msg.channel_id.send_message(&ctx.http, message).await;
            if let Err(err) = result {
                tracing::error!("Error responding to command: {err}");
                bail!(err);
            }
            Ok(())
        }
    }
}

pub async fn play_sound(
    ctx: Context,
    guild_id: GuildId,
    channel_id: ChannelId,
    file: PathBuf,
    config: &BotConfig,
) -> Result<(), anyhow::Error> {
    // Join that channel.
    let manager = songbird::get(&ctx)
        .await
        .ok_or(anyhow!("Songbird not initialized"))?
        .clone();

    let handler_lock = manager.join(guild_id, channel_id).await?;
    let mut handler = handler_lock.lock().await;
    let track_handle = handler.play_only(File::new(file).into());

    // please don't break everyone's eardrums
    track_handle.set_volume(config.volume)?;

    let _ = track_handle.add_event(
        Event::Track(TrackEvent::End),
        LeaveAfterPlaying {
            manager: manager.clone(),
            guild: guild_id,
        },
    );
    Ok(())
}

pub fn choose_sound(soundbank: &Path, category: &str) -> Result<PathBuf, anyhow::Error> {
    let source_dir = soundbank.join(category);
    let children = list_children(source_dir.as_path())?.filter(|f| f.is_file());
    children.choose(&mut rng()).ok_or(anyhow!("no children"))
}

pub fn choose_any_sound(soundbank: &Path) -> Result<PathBuf, anyhow::Error> {
    let pattern = soundbank.join("**/*");
    let pattern_str = pattern
        .to_str()
        .ok_or(anyhow!("Non-UTF8 path not supported"))?;

    let sounds = glob(pattern_str)?
        .filter_map(Result::ok)
        .filter(|f| f.is_file());

    sounds.choose(&mut rng()).ok_or(anyhow!("no children"))
}

pub fn list_categories(soundbank: &Path) -> Result<impl Iterator<Item = String>, anyhow::Error> {
    let results = list_category_directories(soundbank)?.filter_map(|f| get_category_name(&f));
    Ok(results)
}

pub fn list_category_directories(
    soundbank: &Path,
) -> Result<impl Iterator<Item = PathBuf>, anyhow::Error> {
    let results = list_children(soundbank)?
        // only directories
        .filter(|path| path.is_dir());
    Ok(results)
}

pub fn list_children(path: &Path) -> Result<impl Iterator<Item = PathBuf>, anyhow::Error> {
    let pattern = path.join("*");
    let pattern_str = pattern
        .to_str()
        .ok_or(anyhow!("Non-UTF8 path not supported"))?;
    // skip invalid entries
    let results = glob(pattern_str)?.filter_map(Result::ok);
    Ok(results)
}

pub fn get_category_name(path: &Path) -> Option<String> {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|s| s.to_string())
}
