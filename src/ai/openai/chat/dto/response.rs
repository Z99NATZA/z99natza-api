use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct ResponseChoice {
    pub message: Message,
}

#[derive(Deserialize, Debug)]
pub struct OpenAIResponse {
    pub choices: Vec<ResponseChoice>,
}