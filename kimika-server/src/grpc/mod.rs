use kimika_grpc::remote::remote_server::Remote;
use kimika_grpc::remote::{
    get_receivers_response, register_response, EmptyResponse, GetReceiversRequest,
    GetReceiversResponse, ReceiveRequest, ReceiveResponse, RegisterRequest, RegisterResponse,
    SendRequest,
};
use kimika_shared::type_utils::TonicRes;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::{self, Receiver};
use tokio::sync::Mutex;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

type DataReceiver = Receiver<Result<ReceiveResponse, Status>>;

#[derive(Default)]
pub struct RemoteService {
    map: Arc<HashMap<String, Mutex<DataReceiver>>>,
}

impl Clone for RemoteService {
    fn clone(&self) -> Self {
        Self {
            map: Arc::clone(&self.map),
        }
    }
}

#[tonic::async_trait]
impl Remote for RemoteService {
    async fn register(&self, _request: Request<RegisterRequest>) -> TonicRes<RegisterResponse> {
        Ok(Response::new(RegisterResponse {
            id: "foo".to_string(),
            // https://github.com/tokio-rs/prost?tab=readme-ov-file#enumerations
            content_type: register_response::Type::Message as i32,
            name: None,
            size: None,
        }))
    }

    async fn get_receivers(
        &self,
        _request: Request<GetReceiversRequest>,
    ) -> TonicRes<GetReceiversResponse> {
        Ok(Response::new(GetReceiversResponse {
            receivers: vec![get_receivers_response::Receiver {
                alias: "bar".to_string(),
                id: "foo".to_string(),
            }],
        }))
    }

    async fn send(
        &self,
        _request: Request<tonic::Streaming<SendRequest>>,
    ) -> TonicRes<EmptyResponse> {
        // let mut map = self.map.lock().await;
        // let (_tx, rx) = mpsc::channel(4);
        // map.insert("f".to_string(), rx);

        // Ok(Response::new(EmptyResponse {}))
        unimplemented!()
    }

    type ReceiveStream = ReceiverStream<Result<ReceiveResponse, Status>>;
    // type ReceiveStream =
    //     Pin<Box<dyn tokio_stream::Stream<Item = Result<ReceiveResponse, Status>> + Send>>;

    async fn receive(&self, _request: Request<ReceiveRequest>) -> TonicRes<Self::ReceiveStream> {
        // let hashmap = self.map.get("f").unwrap();
        // let sender = hashmap.lock().await;

        // Ok(Response::new(ReceiverStream::new(rx)))
        unimplemented!()
    }
}
