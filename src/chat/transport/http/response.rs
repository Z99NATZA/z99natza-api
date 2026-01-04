use http_body_util::Full;
use hyper::{
    Response, StatusCode,
    body::{Bytes},
};
use serde::Serialize;

use crate::app::AppResult;

pub fn json_response<T: Serialize>(
    status: StatusCode,
    data: &T,
) -> AppResult<Response<Full<Bytes>>> {

    let body = serde_json::to_vec(data)?;

    let resp = Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(Full::new(Bytes::from(body)))
        .unwrap();

    Ok(resp)
}
