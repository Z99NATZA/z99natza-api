pub struct JsonChatRepository {
    base_path: String,
}

impl JsonChatRepository {
    pub fn new(base_path: impl Into<String>) -> Self {
        Self {
            base_path: base_path.into(),
        }
    }

    pub fn path(&self, chat_id: &str) -> String {
        format!("{}/chat_{}.json", self.base_path, chat_id)
    }
}


