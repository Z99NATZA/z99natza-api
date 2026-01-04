use crate::app::AppResult;

#[async_trait::async_trait]
pub trait ChatAi: Send + Sync {
    async fn chat(&self, context: &str) -> AppResult<String>;
}