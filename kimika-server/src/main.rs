mod data;
mod service;

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();

    let listener = TcpListener::bind(addr).await?;

    let server = service::Server::new();

    let server_service = service_fn(move |req| server.clone().handle(req));

    println!("Listening on http://{}", addr);
    loop {
        let (tcp, _) = listener.accept().await?;

        let server_service = server_service.clone();

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(TokioIo::new(tcp), server_service)
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
