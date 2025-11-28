use std::sync::Arc;

use crate::ai::openai::{OpenAiChat, OpenAiConfig};
use crate::ai::{AiChat};
use crate::app::AppResult;
use std::env;

pub enum AiProviderKind {
    OpenAI(OpenAiConfig),
}

pub struct AiProvider {
    inner: Arc<dyn AiChat>,
}

impl AiProviderKind {
    pub fn build(self) -> AiProvider {
        match self {
            AiProviderKind::OpenAI(config) => {
                let client = OpenAiChat::new(config);
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

impl AiProviderKind {
    pub fn from_env() -> Self {
        let cfg = OpenAiConfig {
            api_key: env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY missing"),
            model: env::var("OPENAI_MODEL").unwrap_or("gpt-4o".into()),
            base_url: env::var("OPENAI_BASE_URL")
                .unwrap_or("https://api.openai.com/v1/chat/completions".into()),
        };

        AiProviderKind::OpenAI(cfg)
    }
}
