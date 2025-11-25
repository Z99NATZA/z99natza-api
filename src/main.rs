mod handle;
mod routes;
mod app;
mod http;
mod ai;

use std::net::SocketAddr;

use crate::http::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();
    
    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    Server::run(addr).await.unwrap();
    
    Ok(())
}
