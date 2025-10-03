mod commands;
mod events;
mod util;

use clap::Parser;
use serenity::{
    Client,
    all::{Context, EventHandler, GatewayIntents, Message, Ready, VoiceState},
    async_trait,
};
use songbird::SerenityInit;
use std::path::PathBuf;

pub const COMMAND_PREFIX: &str = "~";

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long, env = "DISCORD_TOKEN")]
    discord_token: String,
    #[arg(
        value_hint = clap::ValueHint::DirPath,
        value_parser = validate_dir_exists
    )]
    sounds_dir: PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&args.discord_token, intents)
        .event_handler(Handler {
            sounds_dir: args.sounds_dir,
        })
        .register_songbird()
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

struct Handler {
    sounds_dir: PathBuf,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        events::on_ready::handle(ctx).await
    }

    async fn message(&self, ctx: Context, msg: Message) {
        events::on_message::handle(ctx, msg, &self.sounds_dir).await
    }

    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        events::on_voice_state_update::handle(ctx, old, new).await
    }
}

fn validate_dir_exists(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    if !path.exists() {
        return Err(format!("Directory doesn't exist: {s}"));
    }
    if !path.is_dir() {
        return Err(format!("Not a directory: {s}"));
    }
    Ok(path)
}
