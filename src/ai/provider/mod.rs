use std::sync::Arc;

use crate::ai::{AiChat};
use crate::ai::openai::OpenAiChat;
use crate::app::AppResult;

pub enum AiProviderKind {
    OpenAI,
}

pub struct AiProvider {
    inner: Arc<dyn AiChat>,
}

impl AiProviderKind {
    pub fn build(self) -> AiProvider {
        match self {
            AiProviderKind::OpenAI => {
                let client = OpenAiChat::new();
                AiProvider { inner: Arc::new(client) }
            }
        }
    }
}

impl AiProvider {
    pub async fn chat(&self, message: &str) -> AppResult<String> {
        self.inner.chat(message).await
    }
}
