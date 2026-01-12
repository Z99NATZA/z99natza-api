use once_cell::sync::Lazy;
use std::sync::Arc;

use crate::modules::ai_chat::usecase::handle_chat::HandleChat;
use super::builder;

static AI_CHAT: Lazy<Arc<HandleChat>> = Lazy::new(|| {
    builder::build()
});

pub fn get() -> Arc<HandleChat> {
    AI_CHAT.clone()
}
