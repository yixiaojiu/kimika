mod get_metadata;
mod get_receivers;
mod post_download;
mod post_metadata;
mod post_register;
mod post_select_metadata;
mod post_upload;
pub mod transfer;

use crate::data;
use crate::utils::hyper_utils;
use crate::utils::types;

use bytes::Bytes;
use hyper::Response;
use std::sync::Arc;
use tklog::async_info;
use tokio::sync::Mutex;

pub struct Server {
    // key id
    receiver: Arc<Mutex<dashmap::DashMap<String, data::Receiver>>>,
    // key: receiver id
    metadata: Arc<Mutex<dashmap::DashMap<String, data::Metadata>>>,
    // key: metadata token
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

    pub async fn handle(
        self,
        req: types::RequestType,
    ) -> Result<
        Response<impl http_body::Body<Data = Bytes, Error = hyper::Error>>,
        Box<dyn std::error::Error + Send + Sync>,
    > {
        let method = req.method();
        let path = req.uri().path();
        async_info!(format!("[{}] [{}]", method, path));

        match (method, path) {
            (&hyper::Method::POST, "/register") => self.post_register(req).await,
            (&hyper::Method::POST, "/upload") => self.post_upload(req).await,
            (&hyper::Method::POST, "/download") => self.post_download(req).await,
            (&hyper::Method::GET, "/receivers") => self.get_receivers(req).await,
            (&hyper::Method::POST, "/metadata") => self.post_metadata(req).await,
            (&hyper::Method::GET, "/metadata") => self.get_metadata(req).await,
            (&hyper::Method::POST, "/metadata/select") => self.post_select_metadata(req).await,
            _ => {
                let mut res = Response::new(hyper_utils::empty());
                *res.status_mut() = hyper::StatusCode::NOT_FOUND;
                Ok(res)
            }
        }
    }
}
