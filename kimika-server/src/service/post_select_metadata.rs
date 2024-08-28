use super::Server;
use crate::utils::hyper_utils;
use crate::utils::types;

use bytes::{Buf, Bytes};
use http_body_util::BodyExt;
use hyper::Response;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct Payload {
    /// receiver id
    id: String,
    /// metadata unique token
    selected_tokens: Vec<String>,
}

#[derive(Serialize)]
struct ResponseBody {
    message: String,
}

impl Server {
    pub async fn post_select_metadata(self, req: types::RequestType) -> types::ResponseType {
        let body = req.collect().await?.aggregate();

        let payload: Payload = serde_json::from_reader(body.reader())?;

        let metadata_guard = self.metadata.lock().await;
        let metadata_entry = metadata_guard.get(&payload.id);

        if let Some(metadata) = metadata_entry {
            metadata
                .selected_metadata_tx
                .send(payload.selected_tokens)
                .await?;
        }

        let body = hyper_utils::full(Bytes::from(
            serde_json::to_string(&ResponseBody {
                message: String::from("ok"),
            })
            .unwrap(),
        ));

        let res = Response::new(body);
        Ok(res)
    }
}
