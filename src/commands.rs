use crate::{
    BotConfig,
    audio::{choose_any_sound, choose_sound, list_categories, play_sound_in_response_to},
    commands,
};
use anyhow::{anyhow, bail};
use serenity::all::{Context, Message, MessageBuilder};
use std::collections::HashSet;

mod help;
mod stats;

pub async fn execute_command(
    ctx: Context,
    msg: Message,
    config: &BotConfig,
) -> Result<(), anyhow::Error> {
    // Ensure it LOOKS like a real command
    let command = match parse_command(&msg.content)? {
        Some(c) => c,
        None => {
            // explicitly do nothing and stop
            return Ok(());
        }
    };

    match command.as_str() {
        "help" => commands::help::run(ctx, msg.author, &config.soundbank).await,
        "stats" => commands::stats::report(ctx, msg, &config.soundbank).await,
        "" => play_any_sound(ctx, msg, config).await,
        category => {
            // Check if it is a valid category.
            let categories: HashSet<String> = list_categories(&config.soundbank)?.collect();

            // If it's a valid category, pick a sound and play it.
            if categories.contains(category) {
                play_sound_from_category(ctx, msg, config, category).await?;
                return Ok(());
            }

            // Otherwise, upload it as a response.
            let result = msg
                .reply(
                    &ctx.http,
                    MessageBuilder::new()
                        .push_bold(&msg.content)
                        .push_line(" is not a valid command.")
                        .build(),
                )
                .await;

            if let Err(err) = result {
                tracing::error!("Unable to reply to an invalid command that was sent: {err}");
                bail!(err);
            }
            Ok(())
        }
    }
}

fn parse_command(content: &str) -> Result<Option<String>, anyhow::Error> {
    // IF there isn't a command prefix then there is no command
    if !content.contains(crate::COMMAND_PREFIX) {
        return Ok(None);
    }

    // Find the LAST token that starts with the prefix.
    let token = content
        .split_whitespace()
        .filter(|f| f.starts_with(crate::COMMAND_PREFIX))
        .next_back()
        .ok_or(anyhow!("no command tokens somehow"))?;

    // Strip off the prefix and figure out what we're doing.
    match token.strip_prefix(crate::COMMAND_PREFIX) {
        Some(cmd) => Ok(Some(cmd.to_string())),
        None => {
            // This should never happen since we filtered for tokens starting with the prefix,
            // but if it does, it's a logic error rather than a user error
            Err(anyhow!(
                "Command token '{}' doesn't start with prefix '{}' - this is a bug",
                token,
                crate::COMMAND_PREFIX
            ))
        }
    }
}

async fn play_any_sound(
    ctx: Context,
    msg: Message,
    config: &BotConfig,
) -> Result<(), anyhow::Error> {
    let sound = choose_any_sound(&config.soundbank)?;
    play_sound_in_response_to(ctx, msg, sound, config).await
}

async fn play_sound_from_category(
    ctx: Context,
    msg: Message,
    config: &BotConfig,
    category: &str,
) -> Result<(), anyhow::Error> {
    let sound = choose_sound(&config.soundbank, category)?;
    play_sound_in_response_to(ctx, msg, sound, config).await
}
