use crate::util::list_categories;
use serenity::all::{Context, Message, MessageBuilder};
use std::{collections::HashSet, path::Path};

pub mod help;
pub mod stats;

pub async fn execute_command(ctx: Context, msg: Message, sounds_dir: &Path) {
    let content = msg.content.clone();

    // Strip off the prefix and figure out what we're doing.
    let command = match content.strip_prefix(crate::COMMAND_PREFIX) {
        Some(cmd) => cmd,
        None => {
            eprintln!("Command could not be parsed: {content}");
            return;
        }
    };

    match command {
        "help" => crate::commands::help::run(ctx, msg.author, sounds_dir).await,
        "stats" => crate::commands::stats::report(ctx, msg, sounds_dir).await,
        cmd => {
            // Check if it is category.
            let categories: HashSet<String> = list_categories(sounds_dir).collect();
            if !categories.contains(cmd) {
                let result = msg
                    .reply(
                        ctx.http,
                        MessageBuilder::new()
                            .push_bold(content)
                            .push_line(" is not a valid command.")
                            .build(),
                    )
                    .await;

                if let Err(err) = result {
                    eprintln!("Unable to reply to an invalid command that was sent: {err}");
                }
                return;
            }

            // It's a valid category, so pick a sound and play it.
        }
    }
}
