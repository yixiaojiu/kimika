use super::grpc::transfer::transfer_server::TransferServer;
use super::grpc::TransferService;
use super::udp::udp_handle;
use tonic::transport::Server;

pub async fn local_receive() -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(async move {
        udp_handle().await.unwrap();
    });

    let transfer_serviece = TransferService::default();
    Server::builder()
        .add_service(TransferServer::new(transfer_serviece))
        .serve("0.0.0.0:3002".parse()?)
        .await?;

    Ok(())
}
