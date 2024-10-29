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

use hyper::Response;
use std::sync::Arc;
use tklog::{async_error, async_info};
use tokio::sync::Mutex;

pub struct Server {
    // key id
    receiver: Arc<Mutex<dashmap::DashMap<String, data::Receiver>>>,
    // key: receiver id
    metadata: Arc<Mutex<dashmap::DashMap<String, data::Metadata>>>,
    // key: metadata token
    transfer: Arc<dashmap::DashMap<String, Mutex<data::Transfer>>>,
}

const EXPIRE_TIME: u64 = 60 * 10;

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
        let method = req.method();
        let path = req.uri().path();
        let log_flag = format!("[{}] [{}]", method, path);
        async_info!(log_flag);

        let res = match (method, path) {
            (&hyper::Method::POST, "/register") => self.post_register(req).await,
            (&hyper::Method::POST, "/upload") => self.post_upload(req).await,
            (&hyper::Method::POST, "/download") => self.post_download(req).await,
            (&hyper::Method::GET, "/receivers") => self.get_receivers(req).await,
            (&hyper::Method::POST, "/metadata") => self.post_metadata(req).await,
            (&hyper::Method::GET, "/metadata") => self.get_metadata(req).await,
            (&hyper::Method::POST, "/metadata/select") => self.post_select_metadata(req).await,
            _ => Ok(Response::builder()
                .status(hyper::StatusCode::NOT_FOUND)
                .body(hyper_utils::empty())
                .unwrap()),
        };

        if let Err(e) = res {
            let error_message = e.to_string();
            async_error!(format!("{} {}", log_flag, error_message));
            return Ok(hyper_utils::rejection_response(error_message));
        }

        res
    }

    pub async fn clear_state(self) {
        async_info!("clear state start");

        let mut clear_receiver_count: u32 = 0;
        let receiver_guard = self.receiver.lock().await;
        receiver_guard.retain(|_, v| {
            let result = v.created.elapsed().as_secs() < EXPIRE_TIME;
            if !result {
                clear_receiver_count += 1;
            }
            result
        });
        drop(receiver_guard);

        let mut clear_metadata_count: u32 = 0;
        let metadata_guard = self.metadata.lock().await;
        metadata_guard.retain(|_, v| {
            let result = v.created.elapsed().as_secs() < EXPIRE_TIME;
            if !result {
                clear_metadata_count += 1;
            }
            result
        });
        drop(metadata_guard);

        async_info!("clear state end");
        async_info!(format!(
            "clear receiver count: {}, clear metadata count: {}",
            clear_receiver_count, clear_metadata_count
        ));
    }
}
