pub mod ai_type;

use std::sync::Arc;

use crate::{ai::AiChat, app::AppResult};

pub struct AiProvider {
    inner: Arc<dyn AiChat>
}

impl AiProvider {
    pub async fn chat(&self, message: &str) -> AppResult<String> {
        self.inner.chat(message).await
    }
}