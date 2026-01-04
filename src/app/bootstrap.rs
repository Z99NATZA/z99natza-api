use std::{env, sync::Arc};

use crate::{
    app::state::AppState,
    chat::{
        ai::{
            manager::AiManager,
            client::openai::OpenAiClient,
        },
        repository::json_file_repo::JsonChatRepository,
        usecase::handle_chat::HandleChat,
    },
};

pub fn bootstrap() -> AppState {
    // ---- infra ----
    let repo = Arc::new(JsonChatRepository::new("data/chat_history"));

    let ai_client = Arc::new(OpenAiClient::new(
        env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY missing"),
        env::var("OPENAI_MODEL").unwrap_or("gpt-4o-mini".into()),
    ));

    let ai = Arc::new(AiManager::new(ai_client));

    // ---- usecase ----
    let handle_chat = Arc::new(HandleChat::new(repo, ai));

    AppState {
        handle_chat,
    }
}
