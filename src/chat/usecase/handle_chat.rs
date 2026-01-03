use std::sync::Arc;

use crate::app::AppResult;
use crate::chat::domain::chat::Chat;
use crate::chat::prompt::context_builder::build_context;
use crate::chat::usecase::dto::chat_message::ChatMessage;
use crate::chat::usecase::dto::chat_request::ChatRequest;

// ---------------------------------------

#[async_trait::async_trait]
pub trait ChatRepository: Send + Sync {
    async fn load(&self, chat_id: &str) -> AppResult<Vec<Chat>>;
    async fn save(&self, chat_id: &str, history: &[Chat]) -> AppResult<()>;
}

#[async_trait::async_trait]
pub trait ChatAi: Send + Sync {
    async fn chat(&self, context: &str) -> AppResult<String>;
}

pub struct HandleChat {
    repo: Arc<dyn ChatRepository>,
    ai: Arc<dyn ChatAi>,
}

// ---------------------------------------

impl HandleChat {
    pub async fn execute(&self, req: ChatRequest) -> AppResult<Vec<ChatMessage>> {
        let mut history = self.repo.load(&req.chat_id).await?;

        history.push(Chat {
            sender: req.sender.clone(),
            message: req.message.clone(),
        });

        let context = build_context(&history);

        let ai_reply = self.ai.chat(&context).await?;

        history.push(Chat {
            sender: "assistant".into(),
            message: ai_reply.clone(),
        });

        self.repo.save(&req.chat_id, &history).await?;

        Ok(vec![
            ChatMessage::user(req),
            ChatMessage::ai(ai_reply),
        ])
    }
}


