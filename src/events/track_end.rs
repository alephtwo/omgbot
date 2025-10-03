use serenity::async_trait;
use songbird::{Event, EventHandler};
use std::sync::Arc;

#[derive(Clone)]
pub struct LeaveAfterPlaying {
    pub manager: Arc<songbird::Songbird>,
    pub guild: serenity::model::id::GuildId,
}

#[async_trait]
impl EventHandler for LeaveAfterPlaying {
    async fn act(&self, _ctx: &songbird::events::EventContext<'_>) -> Option<Event> {
        let _ = self.manager.leave(self.guild).await;
        None
    }
}
