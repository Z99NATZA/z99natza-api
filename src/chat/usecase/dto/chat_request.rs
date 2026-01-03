#[derive(Debug)]
pub struct ChatRequest {
    pub chat_id: String,
    pub sender: String,
    pub message: String,
}
