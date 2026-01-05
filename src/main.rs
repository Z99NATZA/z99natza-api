use std::net::SocketAddr;
use z99natza::infra::http::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();
    dotenv::dotenv().ok();
    
    let addr: SocketAddr = "0.0.0.0:3000".parse()?;
    Server::run(addr).await?;
    
    Ok(())
}
