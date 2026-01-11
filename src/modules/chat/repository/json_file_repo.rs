use std::path::{PathBuf};

use crate::modules::chat::domain::ChatRepository;
use crate::modules::chat::domain::Chat;
use crate::modules::chat::domain::ChatError;
use crate::modules::chat::domain::value::ChatId;

pub struct JsonChatRepository {
    base_path: PathBuf,
}

impl JsonChatRepository {
    pub fn new(base_path: impl Into<PathBuf>) -> Self {
        Self {
            base_path: base_path.into(),
        }
    }

    fn path(&self, chat_id: &ChatId) -> PathBuf {
        self.base_path.join(format!("chat_{}.json", chat_id))
    }
}

#[async_trait::async_trait]
impl ChatRepository for JsonChatRepository {
    async fn load(&self, chat_id: &ChatId) -> Result<Vec<Chat>, ChatError> {
        let path = self.path(chat_id);

        if !tokio::fs::try_exists(&path).await
            .map_err(|_| ChatError::PersistenceFailure)? {
            return Ok(Vec::new());
        }

        let content = tokio::fs::read_to_string(&path)
            .await
            .map_err(|_| ChatError::PersistenceFailure)?;
        let history = serde_json::from_str(&content)
            .map_err(|_| ChatError::PersistenceFailure)?;

        Ok(history)
    }

    async fn save(&self, chat_id: &ChatId, history: &[Chat]) -> Result<(), ChatError> {
        let path = self.path(chat_id);
        let json = serde_json::to_string_pretty(history)
            .map_err(|_| ChatError::PersistenceFailure)?;

        tokio::fs::write(path, json).await
            .map_err(|_| ChatError::PersistenceFailure)?;

        Ok(())
    }
}
