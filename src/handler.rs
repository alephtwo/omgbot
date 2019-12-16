use serenity::model::prelude::Message;
use serenity::prelude::{Context, EventHandler};
use serenity::model::gateway::Ready;
use serenity::model::Permissions;
use std::sync::Arc;
use serenity::http::Http;

pub struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "pong!") {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    fn ready(&self, ctx: Context, payload: Ready) {
        println!("[{}] connected to Discord.", payload.user.name);
        let url = match payload.user.invite_url(&ctx.http, Permissions::empty()) {
            Ok(v) => v,
            Err(why) => {
                eprintln!("Error getting invite url: {:?}", why);
                return;
            }
        };

        println!("Invite URL: {}", url);
    }
}
