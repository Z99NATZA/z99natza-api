use std::net::SocketAddr;

use hyper::{server::conn::http1, service::service_fn};
use hyper_util::rt::{TokioIo, TokioTimer};
use tokio::net::TcpListener;
use crate::routes::api;

pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr: SocketAddr = ([0, 0, 0, 0], 3000).into();

    let listener = TcpListener::bind(addr).await?;
    println!("\n-----------------------------------------");
    println!("App running on: {}", addr);
    println!("-----------------------------------------\n");

    loop {
        let (tcp, _) = listener.accept().await?;

        let io = TokioIo::new(tcp);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .timer(TokioTimer::new())
                .serve_connection(io, service_fn(api::route))
                .await {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}