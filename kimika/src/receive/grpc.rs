use crate::transfer::transfer_server::Transfer;
use crate::transfer::{EmptyResponse, MessageRequest};
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct TransferService {
    tx: tokio::sync::mpsc::Sender<()>,
}

impl TransferService {
    pub fn new(tx: tokio::sync::mpsc::Sender<()>) -> Self {
        TransferService { tx }
    }

    async fn shutdown(&self) {
        self.tx.send(()).await.unwrap();
    }
}

#[tonic::async_trait]
impl Transfer for TransferService {
    async fn send_message(
        &self,
        request: Request<MessageRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        let message_request = request.into_inner();
        println!("Received message: {}", message_request.message);

        self.shutdown().await;
        Ok(Response::new(EmptyResponse {}))
    }
}
