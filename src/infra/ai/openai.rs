use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

use crate::app::AppResult;
use super::client::AiClient;
use crate::modules::ai_chat::prompt::system_prompt::system_prompt;

pub struct OpenAiClient {
    http: Client,
    api_key: String,
    model: String,
}

impl OpenAiClient {
    pub fn new(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            http: Client::new(),
            api_key: api_key.into(),
            model: model.into(),
        }
    }
}

#[async_trait]
impl AiClient for OpenAiClient {
    async fn chat(&self, prompt: &str) -> AppResult<String> {
        let system_prompt = system_prompt();
        
        let resp = self
            .http
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&json!({
                "model": self.model,
                "messages": [
                    { "role": "assistant", "content": system_prompt },
                    { "role": "user", "content": prompt }
                ]
            }))
            .send()
            .await?
            .error_for_status()?
            .json::<serde_json::Value>()
            .await?;

        let content = resp["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();

        Ok(content)
    }
}
