use super::Server;
use crate::data;
use crate::utils::hyper_utils;
use crate::utils::types;

use bytes::Bytes;
use hyper::Response;
use serde::Serialize;

#[derive(Serialize)]
struct ResponseBody {
    receivers: Vec<data::Receiver>,
    message: String,
}

impl Server {
    pub async fn get_receivers(self, _req: types::RequestType) -> types::ResponseType {
        let mut receivers: Vec<data::Receiver> = vec![];

        let receiver_guard = self.receiver.lock().await;

        for item in receiver_guard.iter() {
            receivers.push(item.value().clone());
        }
        drop(receiver_guard);

        let body = hyper_utils::full(Bytes::from(
            serde_json::to_string(&ResponseBody {
                receivers: receivers,
                message: String::from("ok"),
            })
            .unwrap(),
        ));

        let res = Response::new(body);
        Ok(res)
    }
}
