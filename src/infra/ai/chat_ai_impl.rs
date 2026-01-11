use std::sync::Arc;

use crate::app::AppResult;
use crate::modules::ai_chat::ai::port::chat_ai::ChatAi;
use crate::infra::ai::manager::AiManager;

pub struct ChatAiImpl {
    manager: Arc<AiManager>,
}

impl ChatAiImpl {
    pub fn new(manager: Arc<AiManager>) -> Self {
        Self { manager }
    }
}

#[async_trait::async_trait]
impl ChatAi for ChatAiImpl {
    async fn chat(&self, context: &str) -> AppResult<String> {
        self.manager.run(context).await
    }
}
