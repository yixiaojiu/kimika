use super::Server;
use crate::data;
use crate::utils::hyper_utils;
use crate::utils::types;

use bytes::{Buf, Bytes};
use http_body_util::BodyExt;
use hyper::Response;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
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

        let receiver_id = payload.receiver_id;
        let metadata_guard = self.metadata.lock().await;
        let uuid = Uuid::new_v4().to_string();
        let (tx, mut rx) = mpsc::channel(1);
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
            })
            .collect::<Vec<data::MetadataItem>>();

        metadata_guard.insert(
            receiver_id.clone(),
            data::Metadata {
                sender: data::Sender {
                    id: uuid.clone(),
                    alias: payload.alias,
                },
                receiver_id: receiver_id.clone(),
                metadata_list: metadatas,
                selected_metadata_tx: tx,
            },
        );

        drop(metadata_guard);

        // TODO none handle
        let selected_metadata_tokens = rx.recv().await.unwrap();
        let mut selected_metadata_list = Vec::new();
        let metadata_guard = self.metadata.lock().await;
        if let Some(mut metadata) = metadata_guard.get_mut(&receiver_id) {
            metadata
                .metadata_list
                .retain(|v| selected_metadata_tokens.contains(&v.token));
            selected_metadata_list = metadata
                .metadata_list
                .iter()
                .map(|v| ResponseMetadata {
                    id: v.id.clone(),
                    token: v.token.clone(),
                })
                .collect();
        } else {
            // TODO none handle
        }

        drop(metadata_guard);

        let body = hyper_utils::full(Bytes::from(
            serde_json::to_string(&ResponseBody {
                id: uuid,
                selected_metadata_list,
                message: String::from("ok"),
            })
            .unwrap(),
        ));

        let res = Response::new(body);
        Ok(res)
    }
}
