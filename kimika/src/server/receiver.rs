use super::{full, RequestType, ResponseType};

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::Response;
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
pub struct MetadataItem {
    pub id: String,
    pub token: String,
    /// file or message
    pub metadata_type: String,
    pub file_name: Option<String>,
    pub file_type: Option<String>,
    pub size: Option<u64>,
}

pub struct Metadata {
    pub metadata_list: Vec<MetadataItem>,
}

pub struct ReceiverServer {
    metadata: Arc<Mutex<dashmap::DashMap<String, Metadata>>>,
    close_tx: Arc<oneshot::Sender<()>>,
}

impl Clone for ReceiverServer {
    fn clone(&self) -> Self {
        Self {
            metadata: Arc::clone(&self.metadata),
            close_tx: Arc::clone(&self.close_tx),
        }
    }
}

impl ReceiverServer {
    fn new(close_tx: Arc<oneshot::Sender<()>>) -> Self {
        Self {
            metadata: Arc::new(Mutex::new(dashmap::DashMap::new())),
            close_tx,
        }
    }

    pub async fn handle(self, req: RequestType) -> ResponseType {
        let method = req.method();
        let path = req.uri().path();
        Ok(Response::new(full("")))
    }

    pub async fn post_metadata(self, req: RequestType) -> ResponseType {
        Ok(Response::new(full("")))
    }
}

pub async fn start_server(port: u16) -> Result<(), std::io::Error> {
    let address: SocketAddr = ([0, 0, 0, 0], port).into();
    let (close_tx, mut close_rx) = oneshot::channel::<()>();

    let listener = TcpListener::bind(address).await?;
    let server = ReceiverServer::new(Arc::new(close_tx));

    let server_service = service_fn(move |req| server.clone().handle(req));

    loop {
        tokio::select! {
            _ = &mut close_rx => {
                break;
            }
            tcp_icoming = listener.accept() => {
                let (tcp, _) = tcp_icoming?;
                let server_service = server_service.clone();
                tokio::spawn(async move {
                    if let Err(err) = http1::Builder::new()
                        .serve_connection(TokioIo::new(tcp), server_service)
                        .await
                    {
                        eprintln!("Error: {}", err);
                    }
                });
                continue;
            }
        }
    }

    Ok(())
}
