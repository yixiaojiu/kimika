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
    identifier: Option<String>,
}

#[derive(Serialize)]
struct ResponseBody {
    /// receiver id
    id: String,
}

impl Server {
    /// receiver register
    pub async fn post_register(self, req: types::RequestType) -> types::ResponseType {
        let body = req.collect().await?.aggregate();
        let payload: Payload = serde_json::from_reader(body.reader())?;

        let receiver_guard = self.receiver.lock().await;

        let mut receiver_item = receiver_guard
            .iter()
            .find(|item| item.identifier.eq(&payload.identifier));

        let uuid = Uuid::new_v4().to_string();

        let id = if let Some(receiver) = receiver_item.take() {
            receiver.value().id.clone()
        } else {
            let receiver = data::Receiver {
                id: uuid.clone(),
                alias: payload.alias,
                identifier: payload.identifier,
                created: time::Instant::now(),
            };

            receiver_guard.insert(uuid.clone(), receiver);
            uuid
        };

        drop(receiver_item);
        drop(receiver_guard);

        let body = hyper_utils::full(Bytes::from(
            serde_json::to_string(&ResponseBody { id }).unwrap(),
        ));

        let res = Response::new(body);
        Ok(res)
    }
}
