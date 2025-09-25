use std::env;

use serenity::{
    Client,
    all::{
        ActivityData, Context, CreateBotAuthParameters, EventHandler, GatewayIntents, Message,
        Permissions, Ready, Scope, VoiceState,
    },
    async_trait,
};

const COMMAND_PREFIX: &str = "~";

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
        // Generate an invite link
        let builder = CreateBotAuthParameters::new()
            .permissions(
                Permissions::SEND_MESSAGES | Permissions::ATTACH_FILES | Permissions::CONNECT,
            )
            .scopes(&[Scope::Bot])
            .auto_client_id(&ctx.http)
            .await
            .expect("Error generating invite link");

        ctx.set_activity(Some(ActivityData::custom("!help for commands")));
        println!("Invite Link: {}", builder.build());
    }

    async fn message(&self, _ctx: Context, msg: Message) {
        // Unless it's a command there's nothing to do
        if !msg.content.starts_with(COMMAND_PREFIX) {
            return;
        }
    }

    // This code is extremely cursed.
    // It is cursed code.
    // Remove it or keep it at your own peril.
    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        // If it's us, we need to stop.
        if new.user_id == ctx.cache.current_user().id {
            return;
        }

        // Determine which channels we're dealing with.
        let previous_channel = old.and_then(|s| s.channel_id);
        let current_channel = match new.channel_id {
            Some(vs) => vs,
            None => {
                // If the user isn't in a voice channel anymore, then we're done
                return;
            }
        };

        // If the user changed channels, print it out
        if previous_channel != Some(current_channel) {
            println!("Joined Channel: {}", current_channel);
        }
    }
}
