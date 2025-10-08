use crate::BotConfig;
use crate::audio::{choose_sound, play_sound};
use anyhow::anyhow;
use serenity::all::VoiceState;
use serenity::all::{ActivityData, Context, CreateBotAuthParameters, Permissions, Scope};

pub async fn on_ready(ctx: Context) -> Result<(), anyhow::Error> {
    // Generate an invite link
    let builder = CreateBotAuthParameters::new()
        .permissions(Permissions::SEND_MESSAGES | Permissions::ATTACH_FILES | Permissions::CONNECT)
        .scopes(&[Scope::Bot])
        .auto_client_id(&ctx.http)
        .await?;

    ctx.set_activity(Some(ActivityData::custom("!help for commands")));
    println!("Invite Link: {}", builder.build());
    Ok(())
}

// This code is extremely cursed.
// It is cursed code.
// Remove it or keep it at your own peril.
pub async fn on_voice_state_update(
    ctx: Context,
    old: Option<VoiceState>,
    new: VoiceState,
    config: &BotConfig,
) -> Result<(), anyhow::Error> {
    // If it's us, we need to stop.
    if new.user_id == ctx.cache.current_user().id {
        return Ok(());
    }

    // If by some act of god this didn't happen in a guild, stop.
    let guild_id = new.guild_id.ok_or(anyhow!("no guild id"))?;

    // If the user was in a voice channel before, stop.
    // We only want to do something if they join voice for the first time.
    if old.is_some() {
        return Ok(());
    }

    // Figure out which channel the user is in.
    let current_channel = match new.channel_id {
        Some(vs) => vs,
        None => {
            // If the user _isn't_ in a voice channel anymore, then we're done.
            return Ok(());
        }
    };

    println!(
        "{} joined channel {} on guild {}",
        new.user_id, current_channel, guild_id
    );

    let sound = choose_sound(&config.soundbank, "greeting".to_string())?;
    play_sound(ctx, guild_id, current_channel, sound, config).await
}
