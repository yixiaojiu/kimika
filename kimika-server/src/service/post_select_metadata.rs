use super::Server;
use crate::utils::{hyper_utils, types};

use bytes::Buf;
use http_body_util::BodyExt;
use hyper::Response;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Payload {
    /// receiver id
    id: String,
    /// metadata unique token
    selected_tokens: Vec<String>,
}

impl Server {
    pub async fn post_select_metadata(self, req: types::RequestType) -> types::ResponseType {
        let body = req.collect().await?.aggregate();
        let payload: Payload = serde_json::from_reader(body.reader())?;

        let metadata_guard = self.metadata.lock().await;
        let metadata_entry = metadata_guard.get_mut(&payload.id);

        if let Some(mut metadata) = metadata_entry {
            if let Some(tx) = metadata.selected_metadata_tx.take() {
                tx.send(payload.selected_tokens).unwrap();
            }
        } else {
            return Ok(hyper_utils::rejection_response(
                "Cannot find metadata from id",
            ));
        }

        let res = Response::new(hyper_utils::empty());
        Ok(res)
    }
}
