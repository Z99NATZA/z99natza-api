use crate::{ai::OpenAiChat, app::AppResult};

pub enum AiProvider {
    OpenAI,
}

impl AiProvider {
    pub async fn chat(&self, message: &str) -> AppResult<String> {
        match self {
            AiProvider::OpenAI => OpenAiChat::chat(message).await,
        }
    }
}
