use crate::app::AppResult;

#[async_trait::async_trait]
pub trait AiClient: Send + Sync {
    async fn chat(&self, prompt: &str) -> AppResult<String>;
}
