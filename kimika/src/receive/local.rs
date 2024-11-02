use crate::server::receiver::start_server;
use crate::server::udp::listen_boardcast;

use tokio::sync::oneshot;

pub async fn local_receive() -> Result<(), Box<dyn std::error::Error>> {
    let (close_udp_tx, close_udp_rx) = oneshot::channel::<()>();

    tokio::spawn(async move {
        listen_boardcast(close_udp_rx).await.unwrap();
    });

    start_server(close_udp_tx).await?;
    Ok(())
}
