use super::Server;
use crate::data;
use crate::utils::hyper_utils;
use crate::utils::types;

use bytes::Bytes;
use hyper::Response;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct Params {
    /// receiver id
    id: String,
}

#[derive(Serialize)]
struct ResponseBody {
    metadatas: Vec<data::MetadataItem>,
    message: String,
}

impl Server {
    pub async fn get_metadata(self, req: types::RequestType) -> types::ResponseType {
        // TODO none hander
        let query = req.uri().query().unwrap();
        let params: Params = serde_qs::from_str(query)?;

        let metadata_guard = self.metadata.lock().await;
        let metadata_entry = metadata_guard.get(&params.id);

        if let Some(metadata) = metadata_entry {
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
