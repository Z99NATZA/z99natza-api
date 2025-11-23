#![allow(dead_code)]

use thiserror::Error;
use std::convert::Infallible;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("An error occurred")]
    GenericError,
    
    #[error("Env variable error: {0}")]
    EnvVarError(#[from] std::env::VarError),

    #[error("Dotenv loading error: {0}")]
    DotenvError(#[from] dotenv::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("HTTP request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    
    #[error("HTTP error: {0}")]
    HttpError(#[from] hyper::http::Error),
        
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
        
    #[error("Body error: {0}")]
    BodyError(String),
    
    #[error("Hyper error: {0}")]
    HyperError(#[from] hyper::Error),
}

impl From<Infallible> for AppError {
    fn from(infallible: Infallible) -> Self {
        match infallible {}
    }
}

impl AppError {
    pub fn to_response(&self) -> hyper::Response<hyper::body::Bytes> {
        use hyper::StatusCode;
        
        let (status, message) = match self {
            AppError::JsonError(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::HttpError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        
        hyper::Response::builder()
            .status(status)
            .header("content-type", "application/json")
            .body(hyper::body::Bytes::from(
                format!(r#"{{"error":"{}"}}"#, message)
            ))
            .unwrap()
    }
}