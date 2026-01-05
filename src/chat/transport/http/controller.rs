use std::sync::Arc;

use bytes::Bytes;
use http_body_util::Full;
use hyper::{Request, Response, StatusCode, body::Incoming};

use crate::app::{AppResult, state::AppState};
use crate::chat::usecase::dto::chat_request::ChatRequest;
use crate::infra::http::request;
use crate::infra::http::response::json_response;

pub async fn chat_handler(
    state: Arc<AppState>,
    req: Request<Incoming>
) -> AppResult<Response<Full<Bytes>>> {
    let chat_req: ChatRequest = request::json(req).await?;
    let messages = state.handle_chat.execute(chat_req).await?;

    json_response(StatusCode::OK, &messages)
}