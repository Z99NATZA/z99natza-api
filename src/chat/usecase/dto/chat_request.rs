use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatRequest {
    pub chat_id: String,
    pub sender: String,
    pub message: String,
}
