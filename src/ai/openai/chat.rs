use crate::app::AppResult;

pub struct OpenAiChat;

impl OpenAiChat {
    pub async fn chat(message: &str) -> AppResult<String> {
        Ok(format!("Chat response for '{}'", message))
    }
}