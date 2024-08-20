use super::Server;
use crate::data;
use crate::utils::hyper_utils;
use crate::utils::types;

use bytes::{Buf, Bytes};
use http_body_util::BodyExt;
use hyper::Response;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
struct Payload {
    metadata_type: String,
    sender_id: String,
}

#[derive(Serialize)]
struct ResponseBody {
    id: String,
    message: String,
}

impl Server {
    pub async fn post_metadata(self, req: types::RequestType) -> types::ResponseType {
        let body = req.collect().await?.aggregate();
        let payload: Payload = serde_json::from_reader(body.reader())?;

        let uuid = Uuid::new_v4().to_string();

        let body = hyper_utils::full(Bytes::from(
            serde_json::to_string(&ResponseBody {
                id: uuid,
                message: String::from("ok"),
            })
            .unwrap(),
        ));

        let res = Response::new(body);
        Ok(res)
    }
}
