use std::sync::Arc;

use bytes::Bytes;
use http_body_util::Full;
use hyper::{Request, Response, StatusCode, body::Incoming};

use crate::app::AppError;
use crate::app::state::AppState;
use crate::infra::http::request;
use crate::infra::http::response::json_response;
use crate::modules::ai_chat::usecase::dto::ChatRequest;
use crate::modules::ai_chat;
use super::error::map_chat_error;

pub async fn chat_handler(
    _state: Arc<AppState>,
    req: Request<Incoming>
) -> Result<Response<Full<Bytes>>, AppError> {
    let ai_chat = Arc::new(ai_chat::bootstrap());
    
    let chat_req: ChatRequest = request::json(req).await?;
    
    let messages = ai_chat.execute(chat_req).await
        .map_err(map_chat_error)?;

    json_response(StatusCode::OK, &messages)
}
