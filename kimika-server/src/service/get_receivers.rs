use super::Server;
use crate::utils::hyper_utils;
use crate::utils::types;

use bytes::Bytes;
use hyper::Response;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Receiver {
    pub id: String,
    pub alias: String,
}

#[derive(Serialize)]
struct ResponseBody {
    receivers: Vec<Receiver>,
    message: String,
}

impl Server {
    pub async fn get_receivers(self, _req: types::RequestType) -> types::ResponseType {
        let mut receivers: Vec<Receiver> = vec![];

        let receiver_guard = self.receiver.lock().await;

        for item in receiver_guard.iter() {
            let receiver = item.value();
            receivers.push(Receiver {
                id: receiver.id.clone(),
                alias: receiver.alias.clone(),
            });
        }
        drop(receiver_guard);

        let body = hyper_utils::full(Bytes::from(
            serde_json::to_string(&ResponseBody {
                receivers,
                message: String::from("ok"),
            })
            .unwrap(),
        ));

        let res = Response::new(body);
        Ok(res)
    }
}
