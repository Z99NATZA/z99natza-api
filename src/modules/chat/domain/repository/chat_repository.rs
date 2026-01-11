use crate::modules::chat::domain::chat::Chat;
use crate::modules::chat::domain::error::ChatError;
use crate::modules::chat::domain::value::ChatId;

#[async_trait::async_trait]
pub trait ChatRepository: Send + Sync {
    async fn load(&self, chat_id: &ChatId) -> Result<Vec<Chat>, ChatError>;
    async fn save(&self, chat_id: &ChatId, history: &[Chat]) -> Result<(), ChatError>;
}