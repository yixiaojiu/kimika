use crate::transfer::transfer_server::Transfer;
use crate::transfer::{EmptyResponse, MessageRequest};
use tonic::{Request, Response, Status};

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
