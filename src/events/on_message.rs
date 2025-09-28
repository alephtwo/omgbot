use serenity::all::{Context, Message};

pub async fn handle(_ctx: Context, msg: Message) {
    // Unless it's a command there's nothing to do
    if !msg.content.starts_with(crate::COMMAND_PREFIX) {
        return;
    }

    println!("Got command!");
}
