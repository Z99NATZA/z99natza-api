use serde::Deserialize;

use crate::modules::chat::domain::value::ChatId;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatRequest {
    pub chat_id: ChatId,
    pub sender: String,
    pub message: String,
}
