use super::Server;
use crate::data;
use crate::utils::{hyper_utils, types};

use bytes::{Buf, Bytes};
use http_body_util::BodyExt;
use hyper::Response;
use serde::{Deserialize, Serialize};
use std::time;
use tokio::sync::oneshot;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
struct Metadata {
    /// metadata unique id
    id: String,
    /// file or message
    metadata_type: String,
    file_name: Option<String>,
    file_type: Option<String>,
    size: Option<u64>,
}

#[derive(Deserialize, Debug)]
struct Payload {
    receiver_id: String,
    alias: String,
    metadata: Vec<Metadata>,
}

#[derive(Serialize)]
struct ResponseMetadata {
    id: String,
    token: String,
}

#[derive(Serialize)]
struct ResponseBody {
    selected_metadata_list: Vec<ResponseMetadata>,
    /// sender id
    id: String,
    message: String,
}

impl Server {
    pub async fn post_metadata(self, req: types::RequestType) -> types::ResponseType {
        let body = req.collect().await?.aggregate();
        let payload: Payload = serde_json::from_reader(body.reader())?;

        let receiver_guard = self.receiver.lock().await;
        if receiver_guard.contains_key(&payload.receiver_id) {
            receiver_guard.remove(&payload.receiver_id);
        } else {
            return Ok(hyper_utils::rejection_response("Cannot find receiver"));
        }
        drop(receiver_guard);

        let receiver_id = payload.receiver_id;
        let metadata_guard = self.metadata.lock().await;
        let sender_id = Uuid::new_v4().to_string();
        let (tx, rx) = oneshot::channel();
        let metadatas = payload
            .metadata
            .iter()
            .map(|v| data::MetadataItem {
                id: v.id.clone(),
                token: Uuid::new_v4().to_string(),
                metadata_type: v.metadata_type.clone(),
                file_name: v.file_name.clone(),
                file_type: v.file_type.clone(),
                size: v.size.clone(),
                completed: false,
            })
            .collect::<Vec<data::MetadataItem>>();

        metadata_guard.insert(
            receiver_id.clone(),
            data::Metadata {
                sender: data::Sender {
                    id: sender_id.clone(),
                    alias: payload.alias,
                },
                receiver_id: receiver_id.clone(),
                metadata_list: metadatas,
                selected_metadata_tx: Some(tx),
                created: time::Instant::now(),
            },
        );

        drop(metadata_guard);

        // TODO none handle
        let selected_metadata_tokens = rx.await.unwrap();

        let metadata_guard = self.metadata.lock().await;
        let selected_metadata_list =
            if let Some(mut metadata) = metadata_guard.get_mut(&receiver_id) {
                metadata
                    .metadata_list
                    .retain(|v| selected_metadata_tokens.contains(&v.token));
                metadata
                    .metadata_list
                    .iter()
                    .map(|v| ResponseMetadata {
                        id: v.id.clone(),
                        token: v.token.clone(),
                    })
                    .collect()
            } else {
                return Ok(hyper_utils::rejection_response(
                    "Cannot find metadata from receiver id",
                ));
            };

        drop(metadata_guard);

        let body = hyper_utils::full(Bytes::from(
            serde_json::to_string(&ResponseBody {
                id: sender_id,
                selected_metadata_list,
                message: String::from("ok"),
            })
            .unwrap(),
        ));

        let res = Response::new(body);
        Ok(res)
    }
}
