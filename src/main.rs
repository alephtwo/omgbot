use serenity::{
    Client,
    all::{Context, EventHandler, GatewayIntents, Message, Ready, VoiceState},
    async_trait,
};
use std::env;

mod events;

pub const COMMAND_PREFIX: &str = "~";

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be specified");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        events::on_ready::handle(ctx).await
    }

    async fn message(&self, ctx: Context, msg: Message) {
        events::on_message::handle(ctx, msg).await
    }

    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        events::on_voice_state_update::handle(ctx, old, new).await
    }
}
