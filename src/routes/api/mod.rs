#![allow(dead_code)]

use std::sync::Arc;

use http_body_util::Full;
use hyper::{Method, Request, Response};
use bytes::Bytes;
use hyper::body::{Incoming};
use crate::app::AppResult;
use crate::app::state::AppState;
use crate::infra::http::with_cors;
use crate::modules::chat::transport::http::controller::chat_handler;
use serde_json::json;

pub async fn route(
    req: Request<Incoming>,
    state: Arc<AppState>
) -> AppResult<Response<Full<Bytes>>> {
    let method = req.method();
    let uri = req.uri();
    let path = uri.path();
    
    if method == Method::OPTIONS {
        let res = Response::builder()
            .status(200)
            .body(Full::new(Bytes::from_static(b"OK")))
            .unwrap();

        return Ok(with_cors(res));
    }

    let response = match (method, path) {
        (&Method::GET, "/api/") => home().await?,
        (&Method::POST, "/api/ai/chat") => chat_handler(state, req).await?,

        _ => not_found().await?,
    };
    
    Ok(with_cors(response))
}

// temporary
pub async fn home() -> AppResult<Response<Full<Bytes>>> {
    let body = json!({
        "status": "ok",
        "message": "Welcome to API",
        "version": "1.0.0",
        "endpoints": [
            {
                "path": "/api/",
                "method": "GET",
                "description": "API home"
            },
            {
                "path": "/api/ai/chat",
                "method": "POST",
                "description": "AI chat endpoint"
            }
        ]
    });

    let response = Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Full::new(Bytes::from(body.to_string())))
        .unwrap();

    Ok(response)
}

pub async fn not_found() -> AppResult<Response<Full<Bytes>>> {
    let body = json!({
        "status": "error",
        "message": "Endpoint not found",
        "code": 404
    });

    let response = Response::builder()
        .status(404)
        .header("Content-Type", "application/json")
        .body(Full::new(Bytes::from(body.to_string())))
        .unwrap();

    Ok(response)
}