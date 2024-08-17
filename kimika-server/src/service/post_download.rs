use super::Server;
use crate::data;
use crate::utils::types;

use bytes::Bytes;
use http_body_util::{combinators::BoxBody, BodyExt};
use hyper::Response;
use tokio::sync::{oneshot, Mutex};

impl Server {
    pub async fn post_download(self, req: types::RequestType) -> types::ResponseType {
        let (_, req_body) = req.into_parts();

        let transfer_mutex = self
            .transfer
            .entry(String::from("test"))
            .or_insert_with(|| Mutex::new(data::Transfer::new()));

        let mut transfer_guard = transfer_mutex.lock().await;

        let (res_body_tx, res_body_rx) =
            oneshot::channel::<Response<BoxBody<Bytes, hyper::Error>>>();

        match transfer_guard.sender.take() {
            Some(sender) => return Ok(Response::new(sender.req_body.boxed())),
            None => {
                transfer_guard.receiver.replace(data::DataReceiver {
                    res_sender: res_body_tx,
                });
            }
        }
        drop(transfer_guard);
        drop(transfer_mutex);
        Ok(res_body_rx.await.unwrap())
    }
}
