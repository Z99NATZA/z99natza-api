use crate::{app::AppResult, chat::{domain::chat::Chat, repository::json_file_repo::JsonChatRepository}};

#[async_trait::async_trait]
pub trait ChatRepository: Send + Sync {
    async fn load(&self, chat_id: &str) -> AppResult<Vec<Chat>>;
    async fn save(&self, chat_id: &str, history: &[Chat]) -> AppResult<()>;
}

#[async_trait::async_trait]
impl ChatRepository for JsonChatRepository {
    async fn load(&self, chat_id: &str) -> AppResult<Vec<Chat>> {
        let path = self.path(chat_id);

        if !tokio::fs::try_exists(&path).await? {
            return Ok(Vec::new());
        }

        let content = tokio::fs::read_to_string(&path).await?;
        let history = serde_json::from_str(&content)?;
        Ok(history)
    }

    async fn save(&self, chat_id: &str, history: &[Chat]) -> AppResult<()> {
        let path = self.path(chat_id);
        let json = serde_json::to_string_pretty(history)?;
        tokio::fs::write(path, json).await?;
        Ok(())
    }
}