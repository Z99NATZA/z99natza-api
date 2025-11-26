use async_trait::async_trait;
use crate::ai::AiChat;
use crate::app::AppResult;

pub struct OpenAiChat;

impl OpenAiChat {
    pub fn new() -> Self {
        Self {
            
        }
    }
}

#[async_trait]
impl AiChat for OpenAiChat {
    async fn chat(&self, _message: &str) -> AppResult<String> {
        Ok("ผบ. สวัสดีค้าาาาา".to_string())
    }
}
