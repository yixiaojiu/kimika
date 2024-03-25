use kimika_grpc::local::local_server::Local;
use kimika_grpc::local::{EmptyRequest, EmptyResponse, MessageRequest};
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct LocalService {
    tx: tokio::sync::mpsc::Sender<()>,
}

impl LocalService {
    pub fn new(tx: tokio::sync::mpsc::Sender<()>) -> Self {
        LocalService { tx }
    }

    async fn shutdown(&self) {
        self.tx.send(()).await.unwrap();
    }
}

#[tonic::async_trait]
impl Local for LocalService {
    async fn send_message(
        &self,
        request: Request<MessageRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        let message_request = request.into_inner();
        print!("{}", message_request.message);
        Ok(Response::new(EmptyResponse {}))
    }

    async fn close(&self, _: Request<EmptyRequest>) -> Result<Response<EmptyResponse>, Status> {
        self.shutdown().await;
        Ok(Response::new(EmptyResponse {}))
    }
}
