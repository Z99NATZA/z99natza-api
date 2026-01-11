use std::sync::Arc;

use crate::modules::chat::usecase::handle_chat::HandleChat;

pub struct AppState {
    pub handle_chat: Arc<HandleChat>,
}