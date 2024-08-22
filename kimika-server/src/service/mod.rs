mod get_metadata;
mod get_receivers;
mod post_download;
mod post_metadata;
mod post_register;
mod post_select_metadata;
mod post_upload;

use crate::data;
use crate::utils::hyper_utils;
use crate::utils::types;

use hyper::Response;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Server {
    receiver: Arc<Mutex<dashmap::DashMap<String, data::Receiver>>>,
    metadata: Arc<Mutex<dashmap::DashMap<String, data::Metadata>>>,
    transfer: Arc<dashmap::DashMap<String, Mutex<data::Transfer>>>,
}

impl Clone for Server {
    fn clone(&self) -> Self {
        Self {
            metadata: Arc::clone(&self.metadata),
            receiver: Arc::clone(&self.receiver),
            transfer: Arc::clone(&self.transfer),
        }
    }
}

impl Server {
    pub fn new() -> Self {
        Self {
            metadata: Arc::new(Mutex::new(dashmap::DashMap::new())),
            receiver: Arc::new(Mutex::new(dashmap::DashMap::new())),
            transfer: Arc::new(dashmap::DashMap::new()),
        }
    }

    pub async fn handle(self, req: types::RequestType) -> types::ResponseType {
        match (req.method(), req.uri().path()) {
            (&hyper::Method::POST, "/register") => self.post_register(req).await,
            (&hyper::Method::POST, "/upload") => self.post_upload(req).await,
            (&hyper::Method::POST, "/download") => self.post_download(req).await,
            (&hyper::Method::GET, "/receivers") => self.get_receivers(req).await,
            (&hyper::Method::POST, "/metadata") => self.post_metadata(req).await,
            (&hyper::Method::GET, "/metadata") => self.get_metadata(req).await,
            (&hyper::Method::POST, "/select/metadata") => self.post_select_metadata(req).await,
            _ => {
                let mut res = Response::new(hyper_utils::empty());
                *res.status_mut() = hyper::StatusCode::NOT_FOUND;
                Ok(res)
            }
        }
    }
}
