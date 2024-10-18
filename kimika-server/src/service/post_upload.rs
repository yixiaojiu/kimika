use super::transfer::transfer;
use super::Server;
use crate::data;
use crate::utils::{hyper_utils, types};

use hyper::Response;
use serde::Deserialize;
use tokio::sync::oneshot;
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

        let (res_body_tx, res_body_rx) = oneshot::channel::<()>();
        match transfer_guard.receiver.take() {
            Some(receiver) => {
                if let Err(_e) = transfer(
                    data::DataSender {
                        req_body,
                        res_body_tx,
                    },
                    receiver,
                )
                .await
                {
                    // TODO
                }
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

        // TODO: this will produce an error
        let _receive_result = res_body_rx.await;
        Ok(Response::new(hyper_utils::empty()))
    }
}
