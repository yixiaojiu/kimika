use crate::server::udp::UDPPacket;

use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tokio::sync::oneshot;
use tokio::time;

/// `port` is sender listen port
pub async fn broadcast(
    broadcast_addr: SocketAddr,
    port: u16,
    mut close_rx: oneshot::Receiver<()>,
) -> Result<(), std::io::Error> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.set_broadcast(true)?;

    loop {
        tokio::select! {
            _ = &mut close_rx => {
                break;
            }
            _ = time::sleep(time::Duration::from_secs(1)) => {

                socket
                    .send_to(
                        serde_json::to_string(&UDPPacket { port })
                            .unwrap()
                            .as_bytes(),
                        &broadcast_addr,
                    )
                    .await?;
                continue;
            }
        }
    }

    Ok(())
}
