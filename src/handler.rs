use serenity::model::prelude::Message;
use serenity::prelude::{Context, EventHandler};

pub struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "pong!") {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}
