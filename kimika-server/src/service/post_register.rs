use super::Server;
use crate::data;
use crate::utils::hyper_utils;
use crate::utils::types;

use bytes::{Buf, Bytes};
use http_body_util::{BodyExt, Empty};
use hyper::Response;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
struct Payload {
    alias: String,
    role: String,
}

#[derive(Serialize, Debug)]
struct ResponseBody {
    id: String,
}

impl Server {
    pub async fn post_register(self, req: types::RequestType) -> types::ResponseType {
        let body = req.collect().await?.aggregate();
        let payload: Payload = serde_json::from_reader(body.reader())?;

        let uuid = Uuid::new_v4().to_string();

        let body = hyper_utils::full(Bytes::from(
            serde_json::to_string(&ResponseBody { id: uuid }).unwrap(),
        ));

        let res = Response::new(
            Empty::<Bytes>::new()
                .map_err(|never| match never {})
                .boxed(),
        );
        Ok(res)
    }
}
