// use std::sync::Arc;

// use crate::app::AppResult;
// use crate::chat::ai::chat::ChatAi;
// use crate::shared::ai::client::AiClient;

// pub struct AiManager {
//     client: Arc<dyn AiClient>,
// }

// impl AiManager {
//     pub fn new(client: Arc<dyn AiClient>) -> Self {
//         Self { client }
//     }
// }

// #[async_trait::async_trait]
// impl ChatAi for AiManager {
//     async fn chat(&self, context: &str) -> AppResult<String> {
//         self.client.chat(context).await
//     }
// }

use std::sync::Arc;

use crate::app::AppResult;
use crate::shared::ai::client::AiClient;

pub struct AiManager {
    client: Arc<dyn AiClient>,
}

impl AiManager {
    pub fn new(client: Arc<dyn AiClient>) -> Self {
        Self { client }
    }

    pub async fn run(&self, prompt: &str) -> AppResult<String> {
        self.client.chat(prompt).await
    }
}
