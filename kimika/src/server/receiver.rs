use super::{full, rejection_response, RequestType, ResponseType};
use crate::{utils, CONFIG};

use bytes::Buf;
use http_body_util::BodyExt;
use hyper::{server::conn::http1, service::service_fn, Response};
use hyper_util::rt::TokioIo;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{net::SocketAddr, path::PathBuf};
use tokio::fs;
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::{net::TcpListener, sync::oneshot, sync::Mutex};
use tokio_stream::StreamExt;
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
    pub completed: bool,
}

/** ====================================== */

#[derive(Deserialize, Serialize)]
pub struct PayloadMetadataItem {
    pub id: String,
    /// file or message
    pub metadata_type: String,
    pub file_name: Option<String>,
    pub file_type: Option<String>,
    pub size: Option<u64>,
}

#[derive(Deserialize, Serialize)]
pub struct PostRegisterPayload {
    pub alias: String,
    pub metadata_list: Vec<PayloadMetadataItem>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseMetadata {
    pub id: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct PostMetadataResponse {
    pub selected_metadata_list: Vec<ResponseMetadata>,
}

/** ====================================== */

#[derive(Deserialize, Serialize)]
pub struct PostUploadParams {
    pub token: String,
}

/** ====================================== */

pub struct ReceiverServer {
    sender_alias: Arc<Mutex<Option<String>>>,
    metadata_list: Arc<Mutex<Vec<MetadataItem>>>,
    close_server_tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
    close_udp_tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
}

impl Clone for ReceiverServer {
    fn clone(&self) -> Self {
        Self {
            sender_alias: Arc::clone(&self.sender_alias),
            metadata_list: Arc::clone(&self.metadata_list),
            close_server_tx: Arc::clone(&self.close_server_tx),
            close_udp_tx: Arc::clone(&self.close_udp_tx),
        }
    }
}

impl ReceiverServer {
    fn new(close_server_tx: oneshot::Sender<()>, close_udp_tx: oneshot::Sender<()>) -> Self {
        Self {
            metadata_list: Arc::new(Mutex::new(vec![])),
            sender_alias: Arc::new(Mutex::new(None)),
            close_server_tx: Arc::new(Mutex::new(Some(close_server_tx))),
            close_udp_tx: Arc::new(Mutex::new(Some(close_udp_tx))),
        }
    }

    pub async fn handle(self, req: RequestType) -> ResponseType {
        let method = req.method();
        let path = req.uri().path();

        let res = match (method, path) {
            (&hyper::Method::POST, "/metadata") => self.post_metadata(req).await,
            (&hyper::Method::POST, "/upload") => self.post_upload(req).await,
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

    async fn post_metadata(self, req: RequestType) -> ResponseType {
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
                completed: false,
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

        if let Some(close_udp_tx) = self.close_udp_tx.lock().await.take() {
            if let Err(e) = close_udp_tx.send(()) {
                eprintln!("Error: {:?}", e);
            };
        }

        Ok(Response::new(full(serde_json::to_string(
            &PostMetadataResponse {
                selected_metadata_list,
            },
        )?)))
    }

    async fn post_upload(self, req: RequestType) -> ResponseType {
        let (parts, req_body) = req.into_parts();
        // TODO none hander
        let query = parts.uri.query().unwrap();
        let params: PostUploadParams = serde_qs::from_str(query)?;

        let metadata_list_guard = self.metadata_list.lock().await;
        let metadata = match metadata_list_guard.iter().find(|v| v.token == params.token) {
            Some(metadata) => {
                if metadata.completed {
                    return Ok(rejection_response("Metadata already completed"));
                }
                metadata.clone()
            }
            None => {
                return Ok(rejection_response("Metadata check failed"));
            }
        };
        drop(metadata_list_guard);

        if metadata.metadata_type == "file" {
            let mut stream = req_body.into_data_stream();
            let mut pathbuf = PathBuf::from(CONFIG.receiver.save_folder.clone());
            let filename = metadata.file_name.clone().unwrap();
            pathbuf.push(&filename);
            let mut rename_num = 1;
            loop {
                if !pathbuf.exists() {
                    break;
                }
                pathbuf.set_file_name(format!("{}({})", &filename, rename_num));
                rename_num += 1;
            }
            let total_size = metadata.size.unwrap();
            let progreebar = utils::handle::create_progress_bar(total_size, &filename);
            let mut buffer_writer = BufWriter::new(fs::File::create(pathbuf).await?);
            let mut downloaded_size = 0;
            while let Some(chunk) = stream.next().await {
                let chunk = chunk?;
                buffer_writer.write(&chunk).await?;
                downloaded_size += chunk.len() as u64;
                progreebar.set_position(std::cmp::min(downloaded_size, total_size));
            }
            buffer_writer.flush().await?;
            progreebar.finish_with_message(filename);
        } else {
            let body = req_body.collect().await?;
            println!("{}", String::from_utf8_lossy(&body.to_bytes()));
        }

        let mut metadata_list_guard = self.metadata_list.lock().await;
        metadata_list_guard.iter_mut().for_each(|v| {
            if v.token == metadata.token {
                v.completed = true
            }
        });
        let all_completed = metadata_list_guard.iter().all(|v| v.completed == true);
        if all_completed {
            if let Some(close_server_tx) = self.close_server_tx.lock().await.take() {
                if let Err(e) = close_server_tx.send(()) {
                    eprintln!("Error: {:?}", e);
                };
            }
        }

        Ok(Response::new(full("ok")))
    }
}

pub async fn start_server(close_udp_tx: oneshot::Sender<()>) -> Result<(), std::io::Error> {
    let address: SocketAddr = ([0, 0, 0, 0], CONFIG.receiver.port).into();
    let (close_server_tx, mut close_server_rx) = oneshot::channel::<()>();

    let listener = TcpListener::bind(address).await?;
    let server = ReceiverServer::new(close_server_tx, close_udp_tx);

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
