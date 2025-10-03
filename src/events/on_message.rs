use serenity::all::{Context, Message};
use std::path::Path;

pub async fn handle(ctx: Context, msg: Message, sounds_dir: &Path) {
    // Unless it's a command there's nothing to do
    if !msg.content.starts_with(crate::COMMAND_PREFIX) {
        return;
    }

    crate::commands::execute_command(ctx, msg, sounds_dir).await;
}
