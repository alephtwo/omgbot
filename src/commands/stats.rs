use anyhow::{anyhow, bail};
use serenity::all::{Context, Message, MessageBuilder};

use crate::config::BotConfig;

pub async fn report(ctx: Context, msg: Message, config: &BotConfig) -> Result<(), anyhow::Error> {
    let stats = config.soundbank.stats();

    let mut sorted_stats = stats.counts.iter().collect::<Vec<_>>();
    sorted_stats.sort_by_key(|(_, s)| **s);
    let first = sorted_stats.pop().ok_or(anyhow!("no first place"))?;
    let second = sorted_stats.pop().ok_or(anyhow!("no second place"))?;
    let third = sorted_stats.pop().ok_or(anyhow!("no third place"))?;

    let content = MessageBuilder::new()
        .push("There are ")
        .push_bold(stats.sounds.to_string())
        .push(" total sounds across ")
        .push_bold(stats.categories.to_string())
        .push_line(" commands.")
        .push_bold_line("Top 3:")
        .push_line(format!(":first_place: {} ({})", first.0, first.1))
        .push_line(format!(":second_place: {} ({})", second.0, second.1))
        .push_line(format!(":third_place: {} ({})", third.0, third.1))
        .build();

    let result = msg.reply(ctx.http, content).await;
    if let Err(err) = result {
        tracing::error!("Error while responding: {}", err);
        bail!(err);
    }
    Ok(())
}
