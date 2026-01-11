use std::sync::Arc;

use crate::modules::ai_chat::ai::port::chat_ai::ChatAi;
use crate::modules::ai_chat::domain::ChatRepository;
use crate::modules::ai_chat::domain::Chat;
use crate::modules::ai_chat::domain::error::ChatError;
use crate::modules::ai_chat::prompt::context_builder::build_context;
use crate::modules::ai_chat::usecase::dto::chat_message::ChatMessage;
use crate::modules::ai_chat::usecase::dto::chat_request::ChatRequest;

pub struct HandleChat {
    repo: Arc<dyn ChatRepository>,
    ai: Arc<dyn ChatAi>,
}

impl HandleChat {
    pub fn new(
        repo: Arc<dyn ChatRepository>,
        ai: Arc<dyn ChatAi>,
    ) -> Self {
        Self { repo, ai }
    }
    
    pub async fn execute(&self, req: ChatRequest) -> Result<Vec<ChatMessage>, ChatError> {
        let mut history = self.repo.load(&req.chat_id).await?;

        history.push(Chat {
            sender: req.sender.clone(),
            message: req.message.clone(),
        });

        let context = build_context(&history);

        let ai_reply = self.ai.chat(&context).await
            .map_err(|_| ChatError::PersistenceFailure)?;

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
