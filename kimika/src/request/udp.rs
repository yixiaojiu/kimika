use crate::server::udp::UDPPacket;
use crate::CONFIG;

use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tokio::sync::oneshot;
use tokio::time;

pub async fn broadcast(mut close_rx: oneshot::Receiver<()>) -> Result<(), std::io::Error> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    let broadcast_addr: SocketAddr = ([255, 255, 255, 255], CONFIG.sender.receiver_port).into();
    socket.set_broadcast(true)?;

    loop {
        tokio::select! {
            _ = &mut close_rx => {
                break;
            }
            _ = time::sleep(time::Duration::from_secs(1)) => {

                socket
                    .send_to(
                        serde_json::to_string(&UDPPacket { port: CONFIG.sender.port })
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
