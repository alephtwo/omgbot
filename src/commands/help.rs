use crate::audio::list_categories;
use anyhow::bail;
use serenity::all::{Context, CreateMessage, MessageBuilder, User};
use std::path::Path;

pub async fn run(ctx: Context, user: User, soundbank: &Path) -> Result<(), anyhow::Error> {
    let mut categories: Vec<String> = list_categories(soundbank)?.collect();
    categories.sort();

    let mut content = MessageBuilder::new();
    for category in categories {
        content.push("* ").push_mono_line(category);
    }

    let result = user
        .direct_message(ctx.http, CreateMessage::new().content(content.build()))
        .await;

    if let Err(err) = result {
        tracing::error!("Error while responding: {}", err);
        bail!(err);
    }
    Ok(())
}
