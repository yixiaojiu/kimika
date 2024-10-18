use super::transfer::transfer;
use super::Server;
use crate::data;
use crate::utils::types;

use hyper::Response;
use serde::Deserialize;
use tokio::sync::{oneshot, Mutex};

#[derive(Deserialize, Debug)]
struct Params {
    token: String,
    /// receiver id
    id: String,
}

impl Server {
    pub async fn post_download(self, req: types::RequestType) -> types::ResponseType {
        // TODO none hander
        let query = req.uri().query().unwrap();
        let params: Params = serde_qs::from_str(query)?;

        let metadata_guard = self.metadata.lock().await;
        // TODO none hander
        let metadata_entry = metadata_guard.get(&params.id);
        // TODO check
        if let Some(ref metadata) = metadata_entry {
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

        let (res_body_tx, res_body_rx) = oneshot::channel::<Response<types::BodyType>>();

        match transfer_guard.sender.take() {
            Some(sender) => {
                transfer(
                    sender,
                    data::DataReceiver {
                        res_sender: res_body_tx,
                    },
                )
                .await
                .unwrap();
            }
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
