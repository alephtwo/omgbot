mod audio;
mod cli;
mod commands;
mod config;
mod events;

use crate::cli::Cli;
use clap::Parser;
use config::BotConfig;
use serenity::{
    Client,
    all::{Context, EventHandler, GatewayIntents, Message, Ready, VoiceState},
    async_trait,
};
use songbird::SerenityInit;

pub const COMMAND_PREFIX: &str = "!";

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

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
        tracing::error!("Client error: {why:?}");
    }
}

struct Handler {
    config: BotConfig,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        if let Err(err) = events::on_ready(ctx).await {
            tracing::error!("Error in on_ready: {err}");
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if let Err(err) = commands::execute_command(ctx, msg, &self.config).await {
            tracing::error!("Error while executing command: {err}");
        }
    }

    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        if let Err(err) = events::on_voice_state_update(ctx, old, new, &self.config).await {
            tracing::error!("Error in voice_state_update: {err}");
        }
    }
}
