use std::{net::SocketAddr, sync::Arc};

use hyper_util::rt::{TokioIo, TokioTimer};
use tokio::net::TcpListener;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use crate::app::state::AppState;
use crate::routes::api;

pub struct Server;

impl Server {
    pub async fn run(
        addr: SocketAddr,
        state: Arc<AppState>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let listener = TcpListener::bind(addr).await?;
        println!("\n-----------------------------------------");
        println!("App running on: {}", addr);
        println!("-----------------------------------------\n");
        
        loop {
            let (tcp, _) = listener.accept().await?;
    
            let io = TokioIo::new(tcp);
            
            let st = state.clone();
    
            tokio::task::spawn(async move {
                let svc = service_fn(move |req| {
                    let state = st.clone();
                    api::route(req, state)
                });
                
                if let Err(err) = http1::Builder::new()
                    .timer(TokioTimer::new())
                    .serve_connection(io, svc)
                    .await
                {
                    if err.is_timeout() {
                        // 
                    }
                    else {
                        println!("Error serving connection: {:?}", err);
                    }
                }
            });
        }
    }
}