use super::Server;
use crate::data;
use crate::utils::hyper_utils;
use crate::utils::types;

use bytes::{Buf, Bytes};
use http_body_util::BodyExt;
use hyper::Response;
use serde::{Deserialize, Serialize};
use std::time;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
struct Payload {
    alias: String,
}

#[derive(Serialize)]
struct ResponseBody {
    /// receiver id
    id: String,
    message: String,
}

impl Server {
    /// receiver register
    pub async fn post_register(self, req: types::RequestType) -> types::ResponseType {
        let body = req.collect().await?.aggregate();
        let payload: Payload = serde_json::from_reader(body.reader())?;

        let uuid = Uuid::new_v4().to_string();

        let receiver = data::Receiver {
            id: uuid.clone(),
            alias: payload.alias.clone(),
            created: time::Instant::now(),
        };

        let receiver_guard = self.receiver.lock().await;
        receiver_guard.insert(uuid.clone(), receiver);
        drop(receiver_guard);

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
