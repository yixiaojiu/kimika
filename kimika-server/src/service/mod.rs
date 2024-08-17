mod post_download;
mod post_upload;

use crate::data;
use crate::utils::types;

use bytes::Bytes;
use http_body_util::{BodyExt, Empty};
use hyper::Response;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Server {
    sender: Arc<dashmap::DashMap<String, Mutex<data::Sender>>>,
    receiver: Arc<dashmap::DashMap<String, Mutex<data::Receiver>>>,
    metadata: Arc<dashmap::DashMap<String, Mutex<data::Metadata>>>,
    transfer: Arc<dashmap::DashMap<String, Mutex<data::Transfer>>>,
}

impl Clone for Server {
    fn clone(&self) -> Self {
        Self {
            sender: Arc::clone(&self.sender),
            receiver: Arc::clone(&self.receiver),
            metadata: Arc::clone(&self.metadata),
            transfer: Arc::clone(&self.transfer),
        }
    }
}

impl Server {
    pub fn new() -> Self {
        Self {
            sender: Arc::new(dashmap::DashMap::new()),
            receiver: Arc::new(dashmap::DashMap::new()),
            metadata: Arc::new(dashmap::DashMap::new()),
            transfer: Arc::new(dashmap::DashMap::new()),
        }
    }

    pub async fn handle(self, req: types::RequestType) -> types::ResponseType {
        match (req.method(), req.uri().path()) {
            (&hyper::Method::POST, "/upload") => self.post_upload(req).await,
            (&hyper::Method::POST, "/download") => self.post_download(req).await,
            _ => {
                let mut res = Response::new(
                    Empty::<Bytes>::new()
                        .map_err(|never| match never {})
                        .boxed(),
                );
                *res.status_mut() = hyper::StatusCode::NOT_FOUND;
                Ok(res)
            }
        }
    }
}
