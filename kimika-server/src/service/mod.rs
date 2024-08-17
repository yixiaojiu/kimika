use crate::data;
use bytes::Bytes;
use http_body_util::{combinators::BoxBody, BodyExt, StreamBody};
use hyper::{Request, Response};
use std::sync::Arc;
use tokio::sync::{oneshot, Mutex};

pub struct Server {
    transfer: Arc<dashmap::DashMap<String, Mutex<data::Transfer>>>,
}

impl Clone for Server {
    fn clone(&self) -> Self {
        Self {
            transfer: Arc::clone(&self.transfer),
        }
    }
}

impl Server {
    pub fn new() -> Self {
        Self {
            transfer: Arc::new(dashmap::DashMap::new()),
        }
    }

    pub async fn handle(
        self,
        req: Request<hyper::body::Incoming>,
    ) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
        let (req_parts, req_body) = req.into_parts();

        let pipe_mutex = self
            .transfer
            .entry(String::from("test"))
            .or_insert_with(|| Mutex::new(data::Transfer::new()));

        let path = req_parts.uri.path();

        let mut pipe_guard = pipe_mutex.lock().await;

        if path == "/upload" {
            let (res_body_tx, res_body_rx) =
                tokio::sync::mpsc::channel::<Result<http_body::Frame<Bytes>, hyper::Error>>(1);
            match pipe_guard.receiver.take() {
                Some(receiver) => {
                    let aa = Response::new(req_body.boxed());
                    receiver.res_sender.send(aa).unwrap();
                }
                None => {
                    pipe_guard.sender.replace(data::DataSender {
                        req_body,
                        res_body_tx,
                    });
                }
            }
            drop(pipe_guard);
            drop(pipe_mutex);
            let body_stream =
                StreamBody::new(tokio_stream::wrappers::ReceiverStream::new(res_body_rx)).boxed();
            Ok(Response::new(body_stream))
        } else {
            let (res_body_tx, res_body_rx) =
                oneshot::channel::<Response<BoxBody<Bytes, hyper::Error>>>();

            match pipe_guard.sender.take() {
                Some(sender) => return Ok(Response::new(sender.req_body.boxed())),
                None => {
                    pipe_guard.receiver.replace(data::DataReceiver {
                        res_sender: res_body_tx,
                    });
                }
            }
            drop(pipe_guard);
            drop(pipe_mutex);
            Ok(res_body_rx.await.unwrap())
        }
    }
}
