use super::{full, rejection_response, RequestType, ResponseType};
use crate::{utils::select, CONFIG};

use bytes::Buf;
use http_body_util::BodyExt;
use hyper::{server::conn::http1, service::service_fn, Response};
use hyper_util::rt::TokioIo;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::{net::TcpListener, sync::oneshot, sync::Mutex};
use uuid::Uuid;

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

pub struct ReceiverServer {
    sender_alias: Arc<Mutex<Option<String>>>,
    metadata_list: Arc<Mutex<Vec<MetadataItem>>>,
    close_server_tx: Arc<oneshot::Sender<()>>,
}

/** ====================================== */

#[derive(Deserialize)]
struct PayloadMetadataItem {
    id: String,
    /// file or message
    metadata_type: String,
    file_name: Option<String>,
    file_type: Option<String>,
    size: Option<u64>,
}

#[derive(Deserialize)]
struct PostRegisterPayload {
    alias: String,
    metadata_list: Vec<PayloadMetadataItem>,
}

#[derive(Serialize)]
struct ResponseMetadata {
    id: String,
    token: String,
}

#[derive(Serialize)]
struct ResponseBody {
    selected_metadata_list: Vec<ResponseMetadata>,
}

/** ====================================== */

impl Clone for ReceiverServer {
    fn clone(&self) -> Self {
        Self {
            sender_alias: Arc::clone(&self.sender_alias),
            metadata_list: Arc::clone(&self.metadata_list),
            close_server_tx: Arc::clone(&self.close_server_tx),
        }
    }
}

impl ReceiverServer {
    fn new(close_server_tx: Arc<oneshot::Sender<()>>) -> Self {
        Self {
            metadata_list: Arc::new(Mutex::new(vec![])),
            sender_alias: Arc::new(Mutex::new(None)),
            close_server_tx,
        }
    }

    pub async fn handle(self, req: RequestType) -> ResponseType {
        let method = req.method();
        let path = req.uri().path();

        let res = match (method, path) {
            (&hyper::Method::POST, "/metadata") => self.post_metadata(req).await,
            _ => Ok(Response::builder()
                .status(hyper::StatusCode::NOT_FOUND)
                .body(full(""))
                .unwrap()),
        };

        if let Err(e) = res {
            let error_message = e.to_string();
            return Ok(rejection_response(error_message));
        }

        res
    }

    pub async fn post_metadata(self, req: RequestType) -> ResponseType {
        let mut sender_alias_guard = self.sender_alias.lock().await;
        if sender_alias_guard.is_some() {
            return Ok(rejection_response("Receiver already being connected"));
        }

        let body = req.collect().await?.aggregate();
        let payload: PostRegisterPayload = serde_json::from_reader(body.reader())?;

        sender_alias_guard.replace(payload.alias);
        drop(sender_alias_guard);

        // TODO: select metadata

        let metadata_list: Vec<MetadataItem> = payload
            .metadata_list
            .iter()
            .map(|v| MetadataItem {
                id: v.id.clone(),
                token: Uuid::new_v4().to_string(),
                metadata_type: v.metadata_type.clone(),
                file_name: v.file_name.clone(),
                file_type: v.file_type.clone(),
                size: v.size.clone(),
            })
            .collect();

        let selected_metadata_list: Vec<ResponseMetadata> = metadata_list
            .iter()
            .map(|v| ResponseMetadata {
                id: v.id.clone(),
                token: v.token.clone(),
            })
            .collect();

        let mut metadata_list_guard = self.metadata_list.lock().await;
        metadata_list_guard.extend(metadata_list);

        Ok(Response::new(full(serde_json::to_string(&ResponseBody {
            selected_metadata_list,
        })?)))
    }
}

pub async fn start_server() -> Result<(), std::io::Error> {
    let address: SocketAddr = ([0, 0, 0, 0], CONFIG.receiver.port).into();
    let (close_server_tx, mut close_server_rx) = oneshot::channel::<()>();

    let listener = TcpListener::bind(address).await?;
    let server = ReceiverServer::new(Arc::new(close_server_tx));

    let server_service = service_fn(move |req| server.clone().handle(req));

    loop {
        tokio::select! {
            _ = &mut close_server_rx => {
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
