use std::env;
use std::sync::Arc;

use crate::modules::ai_chat::repository::json_file_repo::JsonChatRepository;
use crate::modules::ai_chat::usecase::handle_chat::HandleChat;
use crate::infra::ai::chat_ai_impl::ChatAiImpl;
use crate::infra::ai::manager::AiManager;
use crate::infra::ai::openai::OpenAiClient;

pub fn build() -> Arc<HandleChat> {
    let repo = Arc::new(JsonChatRepository::new("data/chat_history"));

    let ai_client = Arc::new(OpenAiClient::new(
        env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY missing"),
        env::var("OPENAI_MODEL").unwrap_or("gpt-4o-mini".into()),
    ));

    let ai_manager = Arc::new(AiManager::new(ai_client));
    let chat_ai = Arc::new(ChatAiImpl::new(ai_manager));

    Arc::new(HandleChat::new(repo, chat_ai))
}