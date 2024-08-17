use std::net::SocketAddr;
use std::sync::Arc;

use bytes::Bytes;
use http_body_util::{combinators::BoxBody, BodyExt, StreamBody};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use tokio::sync::{mpsc, oneshot, Mutex};

struct DataSender {
    req_body: hyper::body::Incoming,
    #[allow(dead_code)]
    res_body_tx: mpsc::Sender<Result<http_body::Frame<Bytes>, hyper::Error>>,
}

struct DataReceiver {
    res_sender: oneshot::Sender<Response<BoxBody<Bytes, hyper::Error>>>,
}

struct Pipe {
    sender: Option<DataSender>,
    receiver: Option<DataReceiver>,
}

impl Pipe {
    fn new() -> Self {
        Self {
            sender: None,
            receiver: None,
        }
    }
}

struct Server {
    map: Arc<dashmap::DashMap<String, Mutex<Pipe>>>,
}

impl Clone for Server {
    fn clone(&self) -> Self {
        Self {
            map: Arc::clone(&self.map),
        }
    }
}

impl Server {
    fn new() -> Self {
        Self {
            map: Arc::new(dashmap::DashMap::new()),
        }
    }

    pub async fn handle(
        self,
        req: Request<hyper::body::Incoming>,
    ) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
        let (req_parts, req_body) = req.into_parts();

        let pipe_mutex = self
            .map
            .entry(String::from("test"))
            .or_insert_with(|| Mutex::new(Pipe::new()));

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
                    pipe_guard.sender.replace(DataSender {
                        req_body,
                        res_body_tx,
                    });
                }
            }
            drop(pipe_guard);
            drop(pipe_mutex);
            let aa =
                StreamBody::new(tokio_stream::wrappers::ReceiverStream::new(res_body_rx)).boxed();
            let bb = Response::new(aa);
            Ok(bb)
        } else {
            let (res_body_tx, res_body_rx) =
                oneshot::channel::<Response<BoxBody<Bytes, hyper::Error>>>();

            match pipe_guard.sender.take() {
                Some(sender) => return Ok(Response::new(sender.req_body.boxed())),
                None => {
                    pipe_guard.receiver.replace(DataReceiver {
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

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();

    let listener = TcpListener::bind(addr).await?;

    let server = Server::new();

    let server_service = service_fn(move |req| server.clone().handle(req));

    println!("Listening on http://{}", addr);
    loop {
        let (tcp, _) = listener.accept().await?;

        let server_service = server_service.clone();

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(TokioIo::new(tcp), server_service)
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
