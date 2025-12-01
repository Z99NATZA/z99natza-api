use async_trait::async_trait;

use crate::app::AppResult;

#[async_trait]
pub trait AiChat: Send + Sync {
    async fn chat(&self, message: &str) -> AppResult<String>;
}