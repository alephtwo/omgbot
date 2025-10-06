use serenity::all::{Context, Message};
use std::path::Path;

pub async fn handle(ctx: Context, msg: Message, sounds_dir: &Path) {
    crate::commands::execute_command(ctx, msg, sounds_dir).await;
}
