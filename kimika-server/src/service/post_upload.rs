use super::transfer::transfer;
use super::Server;
use crate::data;
use crate::utils::types;

use bytes::Bytes;
use http_body_util::{BodyExt, StreamBody};
use hyper::Response;
use serde::Deserialize;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

#[derive(Deserialize, Debug)]
struct Params {
    token: String,
    /// sender id
    id: String,
    /// receiver id
    receiver: String,
}

impl Server {
    pub async fn post_upload(self, req: types::RequestType) -> types::ResponseType {
        let (parts, req_body) = req.into_parts();
        // TODO none hander
        let query = parts.uri.query().unwrap();
        let params: Params = serde_qs::from_str(query)?;

        let metadata_guard = self.metadata.lock().await;
        // TODO none hander
        let metadata_entry = metadata_guard.get(&params.receiver);
        // TODO check
        if let Some(ref metadata) = metadata_entry {
            let sender_check = metadata.sender.id == params.id;
            let metadata_check = metadata
                .metadata_list
                .iter()
                .any(|v| v.token == params.token);
        } else {
        }
        drop(metadata_entry);
        drop(metadata_guard);

        let transfer_mutex = self
            .transfer
            .entry(params.token.clone())
            .or_insert_with(|| Mutex::new(data::Transfer::new()));

        let mut transfer_guard = transfer_mutex.lock().await;

        let (res_body_tx, res_body_rx) =
            mpsc::channel::<Result<http_body::Frame<Bytes>, hyper::Error>>(1);
        match transfer_guard.receiver.take() {
            Some(receiver) => {
                transfer(
                    data::DataSender {
                        req_body,
                        res_body_tx,
                    },
                    receiver,
                )
                .await
                .unwrap();
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
