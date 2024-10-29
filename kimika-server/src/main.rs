#![allow(dead_code)]

mod data;
mod service;
mod utils;

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use tklog::{async_error, ASYNC_LOG, LEVEL, MODE};
use tokio::net::TcpListener;
use tokio::time;

async fn async_log_init() {
    ASYNC_LOG
        .set_console(true)
        .set_level(LEVEL::Info)
        .set_formatter("{time} {level} {message}\n")
        .set_cutmode_by_time("./log/kimika.log", MODE::DAY, 5, false)
        .await;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    async_log_init().await;

    let addr: SocketAddr = ([0, 0, 0, 0], 3939).into();

    let listener = TcpListener::bind(addr).await?;

    let server = service::Server::new();
    let server_clone = server.clone();

    tokio::spawn(async move {
        const DURATION_TIME: u64 = 2 * 60;

        loop {
            time::sleep(time::Duration::from_secs(DURATION_TIME)).await;
            server_clone.clone().clear_state().await;
        }
    });

    let server_service = service_fn(move |req| server.clone().handle(req));

    println!("Listening on http://{}", addr);
    loop {
        let (tcp, _) = listener.accept().await?;

        let server_service = server_service.clone();

        tokio::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(TokioIo::new(tcp), server_service)
                .await
            {
                async_error!("Error serving connection: ", format!("{:?}", err));
            }
        });
    }
}
