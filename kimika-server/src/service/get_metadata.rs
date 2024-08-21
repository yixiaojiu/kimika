use super::Server;
use crate::data;
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
}

#[derive(Serialize)]
struct ResponseBody {
    metadatas: Vec<data::MetadataItem>,
    message: String,
}

impl Server {
    /// receiver register
    pub async fn get_metadata(self, req: types::RequestType) -> types::ResponseType {
        let body = req.collect().await?.aggregate();
        let payload: Payload = serde_json::from_reader(body.reader())?;

        let metadata_guard = self.metadata.lock().await;
        let aaaa = metadata_guard.get(&payload.id);

        if let Some(metadata) = aaaa {
            let metadatas = metadata.metadatas.clone();
            let body = hyper_utils::full(Bytes::from(
                serde_json::to_string(&ResponseBody {
                    metadatas: metadatas,
                    message: String::from("ok"),
                })
                .unwrap(),
            ));
            let res = Response::new(body);
            Ok(res)
        } else {
            let body = hyper_utils::full("cannot find metadata from id");
            let mut res = Response::new(body);
            *res.status_mut() = hyper::StatusCode::BAD_REQUEST;
            Ok(res)
        }
    }
}
