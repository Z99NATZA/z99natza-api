use chrono::Utc;
use serde::Serialize;

use crate::modules::ai_chat::usecase::dto::chat_request::ChatRequest;


#[derive(Debug, Serialize)]
pub struct ChatMessage {
    pub sender: String,
    pub message: String,
    pub timestamp: String,
}

impl ChatMessage {
    pub fn user(req: ChatRequest) -> Self {
        Self {
            sender: req.sender,
            message: req.message,
            timestamp: Utc::now().to_rfc3339(),
        }
    }

    pub fn ai(message: String) -> Self {
        Self {
            sender: "ai".to_string(),
            message,
            timestamp: Utc::now().to_rfc3339(),
        }
    }
}
