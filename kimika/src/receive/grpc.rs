use tonic::{Request, Response, Status};
use transfer::transfer_server::Transfer;
use transfer::{EmptyResponse, MessageRequest};

pub mod transfer {
    tonic::include_proto!("transfer");
}

#[derive(Debug, Default)]
pub struct TransferService {}

#[tonic::async_trait]
impl Transfer for TransferService {
    async fn send_message(
        &self,
        request: Request<MessageRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        let message_request = request.into_inner();
        println!("Received message: {}", message_request.message);
        Ok(Response::new(EmptyResponse {}))
    }
}
