use std::sync::Arc;

use crate::app::AppResult;
use crate::chat::ai::chat::ChatAi;
use crate::chat::ai::client::client::AiClient;

pub struct AiManager {
    client: Arc<dyn AiClient>,
}

impl AiManager {
    pub fn new(client: Arc<dyn AiClient>) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl ChatAi for AiManager {
    async fn chat(&self, context: &str) -> AppResult<String> {
        self.client.chat(context).await
    }
}
