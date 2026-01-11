use std::{env, sync::Arc};

use crate::app::state::AppState;
use crate::modules::chat::repository::json_file_repo::JsonChatRepository;
use crate::modules::chat::usecase::handle_chat::HandleChat;
use crate::infra::ai::chat_ai_impl::ChatAiImpl;
use crate::infra::ai::manager::AiManager;
use crate::infra::ai::openai::OpenAiClient;

pub fn bootstrap() -> AppState {
    // ---- infra ----
    let repo = Arc::new(JsonChatRepository::new("data/chat_history"));

    let ai_client = Arc::new(OpenAiClient::new(
        env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY missing"),
        env::var("OPENAI_MODEL").unwrap_or("gpt-4o-mini".into()),
    ));
    
    let ai_manager = Arc::new(AiManager::new(ai_client));

    // ---- chat adapter ----
    let chat_ai = Arc::new(ChatAiImpl::new(ai_manager));

    // ---- usecase ----
    let handle_chat = Arc::new(HandleChat::new(repo, chat_ai));

    AppState {
        handle_chat,
    }
}
