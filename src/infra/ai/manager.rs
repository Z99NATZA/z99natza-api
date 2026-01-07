use std::sync::Arc;

use crate::{app::AppResult, infra::ai::client::AiClient};

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
