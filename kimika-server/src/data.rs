use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::Response;
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

pub struct Sender {
    pub id: String,
    pub alias: String,
}

pub struct Receiver {
    pub id: String,
    pub alias: String,
}

pub struct Metadata {
    pub receiver_id: String,
    pub sender_id: String,
    pub id: String,
}

impl Transfer {
    pub fn new() -> Self {
        Self {
            sender: None,
            receiver: None,
        }
    }
}
