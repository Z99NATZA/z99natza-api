use bytes::Bytes;
use http_body_util::Full;
use hyper::{Response, header};

pub fn with_cors(mut res: Response<Full<Bytes>>) -> Response<Full<Bytes>> {
    let headers = res.headers_mut();

    headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
    headers.insert(header::ACCESS_CONTROL_ALLOW_METHODS, "GET, POST, OPTIONS".parse().unwrap());
    headers.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, "*".parse().unwrap());

    res
}