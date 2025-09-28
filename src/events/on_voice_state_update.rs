use serenity::all::{Context, VoiceState};

// This code is extremely cursed.
// It is cursed code.
// Remove it or keep it at your own peril.
pub async fn handle(ctx: Context, old: Option<VoiceState>, new: VoiceState) {
    // If it's us, we need to stop.
    if new.user_id == ctx.cache.current_user().id {
        return;
    }

    // Determine which channels we're dealing with.
    let previous_channel = old.and_then(|s| s.channel_id);
    let current_channel = match new.channel_id {
        Some(vs) => vs,
        None => {
            // If the user isn't in a voice channel anymore, then we're done
            return;
        }
    };

    // If the user changed channels, print it out
    if previous_channel != Some(current_channel) {
        println!("Joined Channel: {}", current_channel);
    }
}
