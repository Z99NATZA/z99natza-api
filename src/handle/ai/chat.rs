use std::{convert::Infallible};
use bytes::Bytes;
use http_body_util::Full;
use hyper::Response;

pub async fn chatv1() -> Result<Response<Full<Bytes>>, Infallible> {
    #[derive(serde::Serialize)]
    struct ChatMessage {
        message: String,
        sender: String,
        timestamp: String,
    }

    let messages = vec![
        ChatMessage {
            message: "Hello".to_string(),
            sender: "user".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        },
        ChatMessage {
            message: "Hi there!".to_string(),
            sender: "ai".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        },
    ];
    let body = serde_json::to_vec(&messages).unwrap();

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json; charset=utf-8")
        .body(Full::new(Bytes::from(body)))
        .unwrap();

    Ok(resp)
}