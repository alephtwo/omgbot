use crate::audio::list_categories;
use serenity::all::{Context, CreateMessage, MessageBuilder, User};
use std::path::Path;

pub async fn run(ctx: Context, user: User, sounds_dir: &Path) {
    let mut categories: Vec<String> = list_categories(sounds_dir).collect();
    categories.sort();

    let mut content = MessageBuilder::new();
    for category in categories {
        content.push("* ").push_mono_line(category);
    }

    let result = user
        .direct_message(ctx.http, CreateMessage::new().content(content.build()))
        .await;

    if let Err(err) = result {
        eprintln!("Error while responding: {}", err)
    }
}
