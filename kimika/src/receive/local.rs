use super::grpc::TransferService;
use super::udp::udp_handle;
use crate::transfer::transfer_server::TransferServer;
use std::net::{Ipv4Addr, SocketAddrV4};
use tonic::transport::Server;

pub async fn local_receive() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: move port to config
    let port: u16 = 3002;
    let address = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port);
    tokio::spawn(async move {
        udp_handle(&address).await.unwrap();
    });

    let transfer_serviece = TransferService::default();
    Server::builder()
        .add_service(TransferServer::new(transfer_serviece))
        .serve(address.into())
        .await?;

    Ok(())
}
