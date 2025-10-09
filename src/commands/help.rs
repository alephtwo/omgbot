use crate::config::BotConfig;
use anyhow::bail;
use serenity::all::{Context, CreateMessage, MessageBuilder, User};

pub async fn run(ctx: Context, user: User, config: &BotConfig) -> Result<(), anyhow::Error> {
    let mut categories: Vec<String> = config.soundbank.categories().collect();
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
