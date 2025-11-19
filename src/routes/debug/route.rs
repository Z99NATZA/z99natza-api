#![allow(unused_assignments)]
#![allow(dead_code)]

use std::convert::Infallible;
use http_body_util::Full;
use hyper::{Method, Request, Response, StatusCode, body::Body, header::CONTENT_TYPE};
use bytes::Bytes;
use serde_json::json;

pub async fn route(
     req: Request<impl Body + std::fmt::Debug>
) -> Result<Response<Full<Bytes>>, Infallible> {
    // Request {
    //     method: GET,
    //     uri: /,
    //     version: HTTP/1.1,
    //     headers: {
    //         "host": "localhost:3000",
    //         "user-agent": "curl/8.14.1",
    //         "accept": "*/*",
    //     },
    //     body: Body(
    //         Empty,
    //     ),
    // }
    println!("req = {:#?}", req);
    println!("req.method = {}", req.method());
    println!("req.uri = {}", req.uri());
    println!("req.uri.path = {}", req.uri().path());
    println!("req.uri.query = {:#?}", req.uri().query());
    println!("req.body = {:#?}", req.body());
    
    let mut message = "";
    
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => message = "GET: '/'",
        (&Method::GET, "/users") => message = "GET: '/users'",
        (&Method::POST, "/users") => message = "POST: '/users'",
        _ => message = "404 Not found"
    };
    
    println!("message = {}", message);
    
    let data = json!({
        "message": message
    });
    
    let json_data = serde_json::to_vec(&data);
    println!("json_data = {:#?}", json_data);
    
    let body = Bytes::from(json_data.unwrap());
    
    println!("body Bytes = {:#?}", body);
    
    Ok(response(body, StatusCode::CREATED))
}

fn response(body: Bytes, status_code: StatusCode) -> Response<Full<Bytes>> {
    Response::builder()
        .status(status_code)
        .header(CONTENT_TYPE, "application/json")
        .body(Full::new(body))
        .unwrap()
}