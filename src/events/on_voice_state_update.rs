use crate::audio::{choose_sound, play_sound};
use serenity::all::{Context, VoiceState};
use std::path::Path;

// This code is extremely cursed.
// It is cursed code.
// Remove it or keep it at your own peril.
pub async fn handle(ctx: Context, old: Option<VoiceState>, new: VoiceState, sounds_dir: &Path) {
    // If it's us, we need to stop.
    if new.user_id == ctx.cache.current_user().id {
        return;
    }

    // If by some act of god this didn't happen in a guild, stop.
    let guild_id = match new.guild_id {
        Some(id) => id,
        None => {
            return;
        }
    };

    // If the user was in a voice channel before, stop.
    // We only want to do something if they join voice for the first time.
    if old.is_some() {
        return;
    }

    // Figure out which channel the user is in.
    let current_channel = match new.channel_id {
        Some(vs) => vs,
        None => {
            // If the user _isn't_ in a voice channel anymore, then we're done.
            return;
        }
    };

    println!(
        "{} joined channel {} on guild {}",
        new.user_id, current_channel, guild_id
    );

    let sound = choose_sound(sounds_dir, "greeting".to_string());
    play_sound(ctx, guild_id, current_channel, sound).await;
}
