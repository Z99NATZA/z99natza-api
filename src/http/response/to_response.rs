use bytes::Bytes;
use http_body_util::Full;
use hyper::{Response, StatusCode};
use hyper::http;

pub struct ToResponse;

impl ToResponse {
    pub fn new(status_code: StatusCode, body: Vec<u8>) -> Result<Response<Full<Bytes>>, http::Error> {
        Response::builder()
            .status(status_code)
            .header("content-type", "application/json; charset=utf-8")
            .body(Full::new(Bytes::from(body)))
    }
}