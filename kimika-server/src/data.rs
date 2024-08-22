use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::Response;
use serde::Serialize;
use tokio::sync::{mpsc, oneshot};

pub struct DataSender {
    pub req_body: hyper::body::Incoming,
    #[allow(dead_code)]
    pub res_body_tx: mpsc::Sender<Result<http_body::Frame<Bytes>, hyper::Error>>,
}

pub struct DataReceiver {
    pub res_sender: oneshot::Sender<Response<BoxBody<Bytes, hyper::Error>>>,
}

pub struct Transfer {
    pub sender: Option<DataSender>,
    pub receiver: Option<DataReceiver>,
}

#[derive(Clone, Serialize)]
pub struct MetadataItem {
    pub id: String,
    /// file or message
    pub metadata_type: String,
    pub file_name: Option<String>,
    pub file_type: Option<String>,
    pub size: Option<u64>,
}

pub struct Sender {
    pub alias: String,
}

pub struct Metadata {
    /// sender alias
    pub sender: Sender,
    pub receiver_id: String,
    pub metadatas: Vec<MetadataItem>,
    pub selected_metadata_tx: mpsc::Sender<Vec<String>>,
}

#[derive(Clone, Serialize)]
pub struct Receiver {
    pub id: String,
    pub alias: String,
}

impl Transfer {
    pub fn new() -> Self {
        Self {
            sender: None,
            receiver: None,
        }
    }
}
