use super::Server;
use crate::data;
use crate::utils::types;

use bytes::Bytes;
use http_body_util::{BodyExt, StreamBody};
use hyper::Response;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

impl Server {
    pub async fn post_upload(self, req: types::RequestType) -> types::ResponseType {
        let (_, req_body) = req.into_parts();

        let transfer_mutex = self
            .transfer
            .entry(String::from("test"))
            .or_insert_with(|| Mutex::new(data::Transfer::new()));

        let mut transfer_guard = transfer_mutex.lock().await;

        let (res_body_tx, res_body_rx) =
            mpsc::channel::<Result<http_body::Frame<Bytes>, hyper::Error>>(1);
        match transfer_guard.receiver.take() {
            Some(receiver) => {
                let aa = Response::new(req_body.boxed());
                receiver.res_sender.send(aa).unwrap();
            }
            None => {
                transfer_guard.sender.replace(data::DataSender {
                    req_body,
                    res_body_tx,
                });
            }
        }
        drop(transfer_guard);
        drop(transfer_mutex);
        let body_stream =
            StreamBody::new(tokio_stream::wrappers::ReceiverStream::new(res_body_rx)).boxed();
        Ok(Response::new(body_stream))
    }
}
