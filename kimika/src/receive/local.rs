use super::udp::udp_handle;
use super::{grpc::TransferService, ReceiveArgs};
use crate::transfer::transfer_server::TransferServer;
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::sync::mpsc::channel;
use tonic::transport::Server;

pub async fn local_receive(args: ReceiveArgs) -> Result<(), Box<dyn std::error::Error>> {
    let address = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), args.port);
    tokio::spawn(async move {
        udp_handle(&address).await.unwrap();
    });

    let (shutdown_sender, mut shutdown_receiver) = channel::<()>(1);

    let transfer_serviece = TransferService::new(shutdown_sender);
    Server::builder()
        .add_service(TransferServer::new(transfer_serviece))
        .serve_with_shutdown(address.into(), async move {
            shutdown_receiver.recv().await.unwrap();
        })
        .await?;

    Ok(())
}
