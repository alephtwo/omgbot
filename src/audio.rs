use crate::BotConfig;
use anyhow::{anyhow, bail};
use serenity::{
    all::{ChannelId, Context, CreateAttachment, CreateMessage, GuildId, Message, MessageBuilder},
    async_trait,
};
use songbird::{Event, EventHandler, TrackEvent, input::File};
use std::{path::Path, sync::Arc};

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
    file: &Path,
    config: &BotConfig,
) -> Result<(), anyhow::Error> {
    // Make sure this command came from a guild
    let guild_id = msg.guild_id.ok_or(anyhow!("Not in a guild"))?;

    // Check if user is in a voice channel without cloning the guild
    let voice_channel = msg
        .guild(&ctx.cache)
        .ok_or(anyhow!("Guild not in cache"))?
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
    file: &Path,
    config: &BotConfig,
) -> Result<(), anyhow::Error> {
    // Join that channel.
    let manager = songbird::get(&ctx)
        .await
        .ok_or(anyhow!("Songbird not initialized"))?
        .clone();

    let handler_lock = manager.join(guild_id, channel_id).await?;
    let mut handler = handler_lock.lock().await;
    let sound = file.to_path_buf();
    let track_handle = handler.play_only(File::new(sound).into());

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
