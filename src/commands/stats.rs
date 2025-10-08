use anyhow::{anyhow, bail};
use serenity::all::{Context, Message, MessageBuilder};
use std::{collections::HashMap, path::Path};

use crate::audio::{get_category_name, list_category_directories, list_children};

pub async fn report(ctx: Context, msg: Message, sounds_dir: &Path) -> Result<(), anyhow::Error> {
    let stats = count_per_category(sounds_dir)?;

    let mut sorted_stats = stats.iter().collect::<Vec<_>>();
    sorted_stats.sort_by_key(|(_, s)| **s);
    let first = sorted_stats.pop().ok_or(anyhow!("no first place"))?;
    let second = sorted_stats.pop().ok_or(anyhow!("no second place"))?;
    let third = sorted_stats.pop().ok_or(anyhow!("no third place"))?;

    let content = MessageBuilder::new()
        .push("There are ")
        .push_bold(stats.values().sum::<usize>().to_string())
        .push(" total sounds across ")
        .push_bold(stats.len().to_string())
        .push_line(" commands.")
        .push_bold_line("Top 3:")
        .push_line(format!(":first_place: {} ({})", first.0, first.1))
        .push_line(format!(":second_place: {} ({})", second.0, second.1))
        .push_line(format!(":third_place: {} ({})", third.0, third.1))
        .build();

    let result = msg.reply(ctx.http, content).await;
    if let Err(err) = result {
        eprintln!("Error while responding: {}", err);
        bail!(err);
    }
    Ok(())
}

fn count_per_category(sounds_dir: &Path) -> Result<HashMap<String, usize>, anyhow::Error> {
    let mut hash: HashMap<String, usize> = HashMap::new();
    for category in list_category_directories(sounds_dir)? {
        let category_name =
            get_category_name(&category).ok_or(anyhow!("failed to get category name"))?;
        hash.insert(category_name, list_children(&category)?.count());
    }
    Ok(hash)
}
