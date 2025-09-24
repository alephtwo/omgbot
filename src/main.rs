use std::env;

use serenity::{
    Client,
    all::{
        Context, CreateBotAuthParameters, EventHandler, GatewayIntents, Message, Permissions,
        Ready, Scope,
    },
    async_trait,
};

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be specified");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
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
            .auto_client_id(ctx.http)
            .await
            .expect("Error generating invite link");
        println!("Invite Link: {}", builder.build());
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content != "~ping" {
            return;
        }

        if let Err(why) = msg.channel_id.say(&ctx.http, "pong!").await {
            println!("Error sending message: {why:?}");
        }
    }
}
