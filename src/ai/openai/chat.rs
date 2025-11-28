use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::ai::AiChat;
use crate::ai::openai::config::OpenAiConfig;
use crate::app::AppResult;

pub struct OpenAiChat {
    config: OpenAiConfig,
}

impl OpenAiChat {
    pub fn new(config: OpenAiConfig) -> Self {
        Self {
            config,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize, Debug)]
struct OpenAIResponse {
    choices: Vec<ResponseChoice>,
}

#[derive(Deserialize, Debug)]
struct ResponseChoice {
    message: Message,
}

#[async_trait]
impl AiChat for OpenAiChat {
    async fn chat(&self, input_chat: &str) -> AppResult<String> {
        let client = reqwest::Client::new();

        println!("{}", self.config.base_url);
        
        let url = self.config.base_url.to_string();

        let res = client
            .post(&url)
            .bearer_auth(&self.config.api_key)
            .json(&serde_json::json!({
                "model": self.config.model,
                "messages": [
                    {
                        "role": "user",
                        "content": input_chat
                    }
                ]
            }))
            .send()
            .await?
            .json::<OpenAIResponse>()
            .await?;

        Ok(res.choices[0].message.content.trim().to_string())
    }
}





