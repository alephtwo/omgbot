use crate::{
    audio::{choose_any_sound, choose_sound, list_categories, play_sound_in_response_to},
    commands,
};
use serenity::all::{Context, Message, MessageBuilder};
use std::{collections::HashSet, path::Path};

mod help;
mod stats;

pub async fn execute_command(ctx: Context, msg: Message, sounds_dir: &Path) {
    // Ensure it LOOKS like a real command
    let command = match parse_command(&msg.content) {
        Some(c) => c,
        None => {
            // explicitly do nothing and stop
            return;
        }
    };

    match command.as_str() {
        "help" => commands::help::run(ctx, msg.author, sounds_dir).await,
        "stats" => commands::stats::report(ctx, msg, sounds_dir).await,
        "" => play_any_sound(ctx, msg, sounds_dir).await,
        category => {
            // Check if it is a valid category.
            let categories: HashSet<String> = list_categories(sounds_dir).collect();
            if !categories.contains(category) {
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
                    eprintln!("Unable to reply to an invalid command that was sent: {err}");
                }
            }

            // It's a valid category, so pick a sound and play it.
            play_sound_from_category(ctx, msg, sounds_dir, category.into()).await;
        }
    }
}

fn parse_command(content: &String) -> Option<String> {
    // IF there isn't a command prefix then there is no command
    if !content.contains(crate::COMMAND_PREFIX) {
        return None;
    }

    // Find the LAST token that starts with the prefix.
    let token = content
        .split(" ")
        .filter(|f| f.starts_with(crate::COMMAND_PREFIX))
        .last()
        .expect("no command tokens somehow");

    // Strip off the prefix and figure out what we're doing.

    match token.strip_prefix(crate::COMMAND_PREFIX) {
        Some(cmd) => Some(cmd.to_string()),
        None => {
            eprintln!("Command could not be parsed: {content}");
            None
        }
    }
}

async fn play_any_sound(ctx: Context, msg: Message, sounds_dir: &Path) {
    let sound = choose_any_sound(sounds_dir);
    play_sound_in_response_to(ctx, msg, sound).await;
}

async fn play_sound_from_category(ctx: Context, msg: Message, sounds_dir: &Path, category: String) {
    let sound = choose_sound(sounds_dir, category);
    play_sound_in_response_to(ctx, msg, sound).await;
}
