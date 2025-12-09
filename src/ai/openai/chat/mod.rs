pub mod dto;

use crate::{ai::{AiChat, OpenAiConfig, openai::chat::dto::OpenAIResponse}, app::AppResult};
use async_trait::async_trait;

pub struct OpenAiChat {
    config: OpenAiConfig
}

impl OpenAiChat {
    pub fn new(config: OpenAiConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl AiChat for OpenAiChat {
    async fn chat(&self, message: &str) -> AppResult<String> {
        let client = reqwest::Client::new();
    
        let url = self.config.base_url.clone();
        let api_key = self.config.api_key.clone();
        let model = self.config.model.clone();
    
        let res = client
            .post(&url)
            .bearer_auth(&api_key)
            .json(&serde_json::json!({
                "model": model,
                "messages": [
                    {
                        "role": "system",
                        "content": "ตอบต่อเนื่องจากประวัติสนทนาล่าสุด (ตอบเป็นข้อความธรรมดา ไม่ต้องเรียนแบบประวัติสนทนา) ตอบสั้นเข้าใจง่าย, คุณเป็นผู้หญิง, อย่างแสดงตัวว่าเป็น AI, ตามน้ำผู้ใช้งาน, เล่นมุขบางครั้ง, ให้อารมณ์เหมือนอยู่ด้วยกัน, สไตล์สาวอนิเมะ ไวฟุ"
                    },
                    {
                        "role": "user",
                        "content": message
                    }
                ]
            }))
            .send()
            .await?
            .json::<OpenAIResponse>()
            .await?;
    
        Ok(res.choices[0].message.content.trim().to_string())
    }
}
