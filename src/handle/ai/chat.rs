use bytes::Bytes;
use http_body_util::Full;
use hyper::Response;
use hyper::Request;
use http_body_util::BodyExt;
use hyper::body::Incoming;
use serde::Deserialize;
use crate::app::AppResult;

#[derive(Debug, Deserialize)]
struct ChatRequest {
    message: String,
}

pub async fn chatv1(
    mut req: Request<Incoming>
) -> AppResult<Response<Full<Bytes>>> {
    let full_body = req.body_mut().collect().await?.to_bytes();
    
    let chat_req: ChatRequest = serde_json::from_slice(&full_body)?;
    println!("{:#?}", chat_req);
    
    #[derive(serde::Serialize)]
    struct ChatMessage {
        message: String,
        sender: String,
        timestamp: String,
    }

    let messages = vec![
        ChatMessage {
            message: chat_req.message,
            sender: "user".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        },
        ChatMessage {
            message: "Hahahahahahahahahaha!".to_string(),
            sender: "ai".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        },
    ];
    let body = serde_json::to_vec(&messages)?;

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json; charset=utf-8")
        .body(Full::new(Bytes::from(body)))?;

    Ok(resp)
}

