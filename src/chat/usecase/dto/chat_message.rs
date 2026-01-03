use chrono::Utc;

use crate::chat::usecase::dto::chat_request::ChatRequest;

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
