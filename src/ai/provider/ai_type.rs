use std::{env, sync::Arc};
use crate::{ai::{OpenAiConfig, openai::chat::core::OpenAiChat, provider::AiProvider}};

pub enum AiType {
    OpenAi(OpenAiConfig),
}

impl AiType {
    pub fn config() -> Self {
        let base_url = env::var("OPENAI_BASE_URL").unwrap_or("https://api.openai.com/v1/chat/completions".into());
        let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY missing");
        let model = env::var("OPENAI_MODEL").unwrap_or("gpt-4o".into());
        
        let config = OpenAiConfig {
            base_url,
            api_key,
            model,
        };
        
        AiType::OpenAi(config)
    }
    
    pub fn build(self) -> AiProvider {
        match self {
            AiType::OpenAi(config) => {
                let chat = OpenAiChat::new(config);
                
                AiProvider {
                    inner: Arc::new(chat)
                }
            }
        }
    }
}