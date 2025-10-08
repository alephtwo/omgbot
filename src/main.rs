mod audio;
mod cli;
mod commands;
mod events;

use crate::cli::Cli;
use clap::Parser;
use serenity::{
    Client,
    all::{Context, EventHandler, GatewayIntents, Message, Ready, VoiceState},
    async_trait,
};
use songbird::SerenityInit;
use std::path::PathBuf;

pub const COMMAND_PREFIX: &str = "!";

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&args.discord_token, intents)
        .event_handler(Handler {
            config: BotConfig {
                soundbank: args.sounds_dir,
                volume: f32::from(args.volume) / 100.0,
            },
        })
        .register_songbird()
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

struct BotConfig {
    soundbank: PathBuf,
    volume: f32,
}

struct Handler {
    config: BotConfig,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        if let Err(err) = events::on_ready(ctx).await {
            eprintln!("Error in on_ready: {err}");
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if let Err(err) = commands::execute_command(ctx, msg, &self.config).await {
            eprintln!("Error while executing command: {err}");
        }
    }

    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        if let Err(err) = events::on_voice_state_update(ctx, old, new, &self.config).await {
            eprintln!("Error in voice_state_update: {err}");
        }
    }
}
