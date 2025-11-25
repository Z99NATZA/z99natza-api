use bytes::Bytes;
use http_body_util::Full;
use hyper::Response;
use hyper::Request;
use hyper::StatusCode;
use hyper::body::Incoming;
use serde::Deserialize;
use crate::ai::AiProvider;
use crate::app::AppResult;
use crate::http::ToResponse;
use crate::http::request::FullBody;

#[derive(Debug, Deserialize)]
struct ChatRequest {
    message: String,
    sender: String,
}

pub async fn chatv1(
    req: Request<Incoming>
) -> AppResult<Response<Full<Bytes>>> {
    let full_body = FullBody::new(req).await?;
    
    let chat_req: ChatRequest = serde_json::from_slice(&full_body)?;
    
    let provider = AiProvider::OpenAI;
    let ai_message = provider.chat(chat_req.message.as_str()).await?;

    #[derive(serde::Serialize)]
    struct ChatMessage {
        message: String,
        sender: String,
        timestamp: String,
    }

    let messages = vec![
        ChatMessage {
            message: chat_req.message,
            sender: chat_req.sender,
            timestamp: chrono::Utc::now().to_rfc3339(),
        },
        ChatMessage {
            message: ai_message,
            sender: "ai".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        },
    ];
    
    let body = serde_json::to_vec(&messages)?;

    let resp = ToResponse::new(StatusCode::OK, body)?;

    Ok(resp)
}

