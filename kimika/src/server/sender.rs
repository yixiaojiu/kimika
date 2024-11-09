use super::{full, RequestType, ResponseType};
use crate::CONFIG;

use bytes::Buf;
use http_body_util::BodyExt;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::Response;
use hyper_util::rt::TokioIo;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::sync::{mpsc, oneshot};

#[derive(Deserialize, Serialize)]
pub struct Payload {
    pub alias: String,
    pub port: u16,
}

#[derive(Debug, Clone)]
pub struct Receiver {
    pub address: SocketAddr,
    pub alias: String,
}

async fn handle(req: RequestType, address: SocketAddr, tx: mpsc::Sender<Receiver>) -> ResponseType {
    let (parts, incoming) = req.into_parts();

    if !(parts.uri.path() == "/register" && parts.method == hyper::Method::POST) {
        return Ok(Response::builder()
            .status(hyper::StatusCode::NOT_IMPLEMENTED)
            .body(full("not implemented"))?);
    }

    let buf = incoming.collect().await?.aggregate();
    let body: Payload = serde_json::from_reader(buf.reader())?;

    if tx.is_closed() {
        return Ok(Response::builder()
            .status(hyper::StatusCode::NOT_ACCEPTABLE)
            .body(full("server will be shut down soon"))?);
    }

    tx.send(Receiver {
        address: SocketAddr::new(address.ip(), body.port),
        alias: body.alias.clone(),
    })
    .await?;

    Ok(Response::new(full("ok")))
}

pub async fn start_server(
    tx: mpsc::Sender<Receiver>,
    mut close_rx: oneshot::Receiver<()>,
) -> Result<(), std::io::Error> {
    let addr: SocketAddr = ([0, 0, 0, 0], CONFIG.sender.port).into();
    let listener = TcpListener::bind(addr).await?;

    loop {
        tokio::select! {
            _ = &mut close_rx => {
                break;
            }
            tcp_icoming = listener.accept() => {
                let (tcp, address) = tcp_icoming?;
                let tx_clone = tx.clone();
                tokio::spawn(async move {
                    let _ = http1::Builder::new()
                        .serve_connection(
                            TokioIo::new(tcp),
                            service_fn(|req| {
                                let receiver_tx = tx_clone.clone();
                                handle(req, address, receiver_tx)
                            }),
                        )
                        .await;
                });
                continue;
            }
        }
    }

    Ok(())
}
