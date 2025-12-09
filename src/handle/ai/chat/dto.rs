use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatRequest {
    pub message: String,
    pub sender: String,
    pub chat_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    pub sender: String,
    pub message: String,
}

#[derive(serde::Serialize)]
pub struct ChatMessage {
    pub message: String,
    pub sender: String,
    pub timestamp: String,
}