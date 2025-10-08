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
        commands::execute_command(ctx, msg, &self.sounds_dir).await;
    }

    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        events::on_voice_state_update::handle(ctx, old, new, &self.sounds_dir).await
    }
}
