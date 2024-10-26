use super::ReceiveArgs;
use crate::config;
use crate::server::udp::listen_boardcast;

use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::sync::oneshot;

pub async fn local_receive(
    _args: &ReceiveArgs,
    config: &config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let (close_udp_tx, close_udp_rx) = oneshot::channel::<()>();

    let address = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), config.receiver.port);

    tokio::spawn(async move {
        listen_boardcast(&address, close_udp_rx).await.unwrap();
    });

    Ok(())
}
