use std::fmt::Debug;
use bytes::Bytes;
use http_body_util::Full;
use hyper::Response;
use hyper::Request as HyperRequest;
use hyper::body::Body;

use crate::app::AppResult;

pub async fn chatv1(
    req: HyperRequest<impl Body + Debug>
) -> AppResult<Response<Full<Bytes>>> {
    println!("{:#?}", req);
    
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

