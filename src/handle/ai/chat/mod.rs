pub mod dto;
pub use dto::ChatRequest;
pub use dto::Chat;
pub use dto::ChatMessage;

use std::sync::Arc;
use bytes::Bytes;
use http_body_util::Full;
use hyper::Response;
use hyper::Request;
use hyper::StatusCode;
use hyper::body::Incoming;

use crate::app::AppResult;
use crate::app::state::AppState;
use crate::http::ToResponse;
use crate::http::request;
use tokio::fs;
use std::path::Path;

pub async fn chat_handle(
    state: Arc<AppState>,
    req: Request<Incoming>
) -> AppResult<Response<Full<Bytes>>> {
    let chat_req: ChatRequest = request::json(req).await?;
    
    // chat_id
    let chat_id = chat_req.chat_id.clone();
    
    let chat_data = Chat {
        sender: chat_req.sender.clone(),
        message: chat_req.message.clone(),
    };
    
    let json_file_name = format!("chat_{}.json", chat_id);
    let json_path = format!("data/chat_history/{}", json_file_name);
    
    // pull chat history
    let mut history: Vec<Chat> = if Path::new(&json_path).exists() {
        let content = fs::read_to_string(&json_path).await?;
        serde_json::from_str(&content)?
    } 
    else {
        Vec::new()
    };
    
    // add current user message
    history.push(chat_data);
    
    let mut context = String::new();
    
    // set context
    for h in history.iter() {
        context = format!("{} {}: {}\n", context, h.sender, h.message);
    }
    
    // call ai
    let ai_message = state.ai.chat(context.as_str()).await?;
    
    let chat_data = Chat {
        sender: "assistant".to_string(),
        message: ai_message.clone(),
    };
    
    // add current ai reply
    history.push(chat_data);
    
    let json = serde_json::to_string_pretty(&history)?;
    tokio::fs::write(&json_path, json).await?;

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

