use super::ReceiveArgs;
use crate::config;
use crate::server::udp::listen_boardcast;

use tokio::sync::oneshot;

pub async fn local_receive(
    _args: &ReceiveArgs,
    config: &config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let (close_udp_tx, close_udp_rx) = oneshot::channel::<()>();

    let alias = config.alias.clone();
    let port = config.receiver.port;

    tokio::spawn(async move {
        listen_boardcast(close_udp_rx, alias, port).await.unwrap();
    });

    Ok(())
}
