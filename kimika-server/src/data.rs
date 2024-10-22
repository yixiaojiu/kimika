use crate::utils::types;

use hyper::Response;
use serde::Serialize;
use std::time;
use tokio::sync::oneshot;

pub struct DataSender {
    pub req_body: hyper::body::Incoming,
    pub res_body_tx: oneshot::Sender<()>,
}

pub struct DataReceiver {
    pub res_sender: oneshot::Sender<Response<types::BodyType>>,
}

pub struct Transfer {
    pub sender: Option<DataSender>,
    pub receiver: Option<DataReceiver>,
}

#[derive(Clone, Debug, Serialize)]
pub struct MetadataItem {
    pub id: String,
    pub token: String,
    /// file or message
    pub metadata_type: String,
    pub file_name: Option<String>,
    pub file_type: Option<String>,
    pub size: Option<u64>,
    /// whether completed
    pub completed: bool,
}

#[derive(Debug)]
pub struct Sender {
    pub alias: String,
    pub id: String,
}

#[derive(Debug)]
pub struct Metadata {
    /// sender alias
    pub sender: Sender,
    pub receiver_id: String,
    pub metadata_list: Vec<MetadataItem>,
    pub selected_metadata_tx: Option<oneshot::Sender<Vec<String>>>,
    pub created: time::Instant,
}

#[derive(Debug)]
pub struct Receiver {
    pub id: String,
    pub alias: String,

    /// Unique identifier from client, such as: mac address
    ///
    /// Preventing duplicate insertion
    pub identifier: Option<String>,

    pub created: time::Instant,
}

impl Transfer {
    pub fn new() -> Self {
        Self {
            sender: None,
            receiver: None,
        }
    }
}
