use super::udp::listen_boardcast;
use super::{local_grpc::LocalService, ReceiveArgs};
use crate::config;
use kimika_grpc::local::local_server::LocalServer;
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::sync::mpsc::channel;
use tonic::transport::Server;

pub async fn local_receive(
    _args: &ReceiveArgs,
    config: &config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let alias = config.alias.clone();
    let save_folder = std::path::PathBuf::from(config.receiver.save_folder.clone());

    let address = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), config.receiver.port);

    tokio::spawn(async move {
        listen_boardcast(&address, &alias).await.unwrap();
    });

    let (shutdown_sender, mut shutdown_receiver) = channel::<()>(1);

    let local_serviece = LocalService::new(shutdown_sender, &save_folder);
    Server::builder()
        .add_service(LocalServer::new(local_serviece))
        .serve_with_shutdown(address.into(), async move {
            shutdown_receiver.recv().await.unwrap();
        })
        .await?;

    Ok(())
}
